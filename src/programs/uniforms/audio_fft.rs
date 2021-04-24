use nannou::prelude::*;
use ringbuf::{Consumer, RingBuffer};
use rustfft::{num_complex::Complex, FftPlanner};
use std::fmt;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use crate::programs::config;
use crate::programs::uniforms::audio_source;
use crate::programs::uniforms::base::Bufferable;
use crate::util;

const DEFAULT_SPECTRUM_SIZE: usize = 32;
const WINDOW_SIZE: usize = 1024;

pub struct AudioFftUniforms {
    pub smoothing: f32,
    pub spectrum_texture: wgpu::Texture,
    pub spectrum_size: usize,

    audio_channel_tx: Option<Sender<audio_source::AudioMessage>>,
    fft_thread: Option<std::thread::JoinHandle<()>>,
    spectrum_consumer: Option<Consumer<Vec<f32>>>,
    spectrum: Vec<f32>,
}

impl fmt::Debug for AudioFftUniforms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AudioFftUniforms")
    }
}

impl Bufferable for AudioFftUniforms {
    fn textures(&self) -> Vec<&wgpu::Texture> {
        vec![&self.spectrum_texture]
    }
}

fn float_as_bytes(data: &f32) -> &[u8] {
    unsafe { wgpu::bytes::from(data) }
}

fn floats_as_byte_vec(data: &Vec<f32>) -> Vec<u8> {
    let mut bytes = vec![];
    data.iter().for_each(|f| bytes.extend(float_as_bytes(f)));
    bytes
}

impl AudioFftUniforms {
    pub fn new(device: &wgpu::Device, spectrum_size_opt: Option<usize>) -> Self {
        let spectrum_size = spectrum_size_opt.unwrap_or(DEFAULT_SPECTRUM_SIZE);
        let spectrum_texture = util::create_texture(
            device,
            [spectrum_size as u32, 1],
            wgpu::TextureFormat::R32Float,
        );

        Self {
            audio_channel_tx: None,
            fft_thread: None,
            smoothing: 0.5,
            spectrum_consumer: None,
            spectrum_texture,
            spectrum: vec![0.0; spectrum_size],
            spectrum_size,
        }
    }

    pub fn configure(&mut self, settings: &Option<config::ProgramSettings>) {
        self.smoothing = 0.5;

        if let Some(cnfg) = settings {
            if let Some(smoothing) = cnfg.audio_fft_smoothing {
                self.smoothing = smoothing;
            }
        }
    }

    pub fn start_session(&mut self, audio_source: &mut audio_source::AudioSource) {
        let (audio_channel_tx, audio_channel_rx) = channel();
        audio_source.subscribe(String::from("audio_fft"), audio_channel_tx.clone());
        self.audio_channel_tx = Some(audio_channel_tx);

        // setup the FFT
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(WINDOW_SIZE);
        let hanning_window = apodize::hanning_iter(WINDOW_SIZE).collect::<Vec<f64>>();

        // create a ring buffer for spectrum results
        let ring_buffer = RingBuffer::<Vec<f32>>::new(2);
        let (mut producer, consumer) = ring_buffer.split();
        producer.push(vec![0.0; self.spectrum_size]).unwrap();
        self.spectrum_consumer = Some(consumer);

        let spec_group_size = (WINDOW_SIZE / 2) / self.spectrum_size;
        let spectrum_size = self.spectrum_size.clone();

        self.fft_thread = Some(thread::spawn(move || {
            let mut frames = vec![];
            frames.push(vec![0.0; audio_source::FRAME_SIZE]);
            frames.push(vec![0.0; audio_source::FRAME_SIZE]);

            for message in audio_channel_rx.iter() {
                match message {
                    audio_source::AudioMessage::Data(frame) => {
                        // add new spectrum to memory and build the window
                        frames.remove(0);
                        frames.push(frame);
                        let mut window = frames
                            .clone()
                            .into_iter()
                            .flatten()
                            .enumerate()
                            .take(WINDOW_SIZE)
                            .map(|(i, s)| Complex {
                                re: s * hanning_window[i] as f32,
                                im: 0.0,
                            })
                            .collect::<Vec<Complex<f32>>>();

                        // perform the fft to get the spectrum
                        fft.process(&mut window[..]);
                        let spectrum = window
                            .iter()
                            .take(WINDOW_SIZE / 2)
                            .map(|s| s.norm())
                            .collect::<Vec<f32>>();

                        // downsample the spectrum
                        let mut reduced_spectrum = vec![0.0; spectrum_size];
                        for i in 0..spectrum_size {
                            let mut sum = 0.0;
                            for j in 0..spec_group_size {
                                sum += spectrum[(i * spec_group_size) + j];
                            }
                            reduced_spectrum[i] = sum / spec_group_size as f32;
                        }

                        producer.push(reduced_spectrum).ok();
                    }
                    audio_source::AudioMessage::Close | audio_source::AudioMessage::Error(_) => {
                        break
                    }
                }
            }
        }));
    }

    pub fn end_session(&mut self, audio_source: &mut audio_source::AudioSource) {
        audio_source.unsubscribe(String::from("audio_fft"));

        if let Some(channel) = &self.audio_channel_tx {
            channel.send(audio_source::AudioMessage::Close).unwrap();
        }

        if let Some(handle) = self.fft_thread.take() {
            handle.join().unwrap();
        }
    }

    pub fn update(&mut self) {
        if let Some(mut c) = self.spectrum_consumer.take() {
            let popped = c.pop();
            self.spectrum_consumer = Some(c);

            if let Some(f) = popped {
                for (i, &sample) in f.iter().enumerate().take(self.spectrum_size) {
                    self.spectrum[i] = audio_source::lerp(self.spectrum[i], sample, self.smoothing);
                }
            }
        }
    }

    pub fn update_texture(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let bytes = floats_as_byte_vec(&self.spectrum);
        self.spectrum_texture
            .upload_data(device, encoder, &bytes[..]);
    }
}
