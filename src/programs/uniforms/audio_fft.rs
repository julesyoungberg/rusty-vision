use nannou::prelude::*;
use ringbuf::{Consumer, RingBuffer};
use rustfft::{num_complex::Complex, FftPlanner};
use std::sync::mpsc::{channel, Sender};
use std::thread;

use crate::programs::config;
use crate::programs::uniforms::audio_source;
use crate::programs::uniforms::base::Bufferable;

const SPECTRUM_SIZE: usize = 32;
const WINDOW_SIZE: usize = 1024;

pub struct AudioFftUniforms {
    pub smoothing: f32,

    fft_thread: Option<std::thread::JoinHandle<()>>,
    spectrum_consumer: Option<Consumer<Vec<f32>>>,
    spectrum_texture: wgpu::Texture,
    spectrum: Vec<f32>,
}

impl Bufferable for AudioFftUniforms {
    fn textures(&self) -> Vec<&wgpu::Texture> {
        vec![&self.spectrum_texture]
    }
}

impl AudioFftUniforms {
    pub fn new(device: &wgpu::Device) -> Self {
        let spectrum_texture = wgpu::TextureBuilder::new()
            .size([SPECTRUM_SIZE as u32, 1])
            .format(wgpu::TextureFormat::R32Float)
            .usage(wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED)
            .build(device);

        Self {
            fft_thread: None,
            smoothing: 0.5,
            spectrum_consumer: None,
            spectrum_texture,
            spectrum: vec![0.0; SPECTRUM_SIZE],
        }
    }

    pub fn set_defaults(&mut self, defaults: &Option<config::ProgramDefaults>) {
        self.smoothing = 0.5;

        if let Some(cnfg) = defaults {
            if let Some(smoothing) = cnfg.audio_fft_smoothing {
                self.smoothing = smoothing;
            }
        }
    }

    pub fn start_session(&mut self) -> Sender<audio_source::AudioMessage> {
        let (audio_channel_tx, audio_channel_rx) = channel();

        // setup the FFT
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(WINDOW_SIZE);
        let hanning_window = apodize::hanning_iter(WINDOW_SIZE).collect::<Vec<f64>>();

        // create a ring buffer for spectrum results
        let ring_buffer = RingBuffer::<Vec<f32>>::new(2);
        let (mut producer, consumer) = ring_buffer.split();
        producer.push(vec![0.0; SPECTRUM_SIZE]).unwrap();
        self.spectrum_consumer = Some(consumer);

        let spec_group_size = (WINDOW_SIZE / 2) / SPECTRUM_SIZE;

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
                        let mut reduced_spectrum = vec![0.0; SPECTRUM_SIZE];
                        for i in 0..SPECTRUM_SIZE {
                            let mut sum = 0.0;
                            for j in 0..spec_group_size {
                                sum += spectrum[(i * spec_group_size) + j];
                            }
                            reduced_spectrum[i] = sum / spec_group_size as f32;
                        }

                        producer.push(reduced_spectrum).ok();
                    }
                    audio_source::AudioMessage::Close(()) => break,
                }
            }
        }));

        audio_channel_tx
    }

    pub fn end_session(&mut self) {
        if let Some(handle) = self.fft_thread.take() {
            handle.join().unwrap();
        }
    }

    pub fn update(&mut self) {
        if let Some(mut c) = self.spectrum_consumer.take() {
            let popped = c.pop();
            self.spectrum_consumer = Some(c);

            if let Some(f) = popped {
                for (i, &sample) in f.iter().enumerate().take(SPECTRUM_SIZE) {
                    self.spectrum[i] = audio_source::lerp(self.spectrum[i], sample, self.smoothing);
                }
            }
        }
    }

    pub fn update_texture(
        &self,
        device: &wgpu::Device,
        encoder: &mut nannou::wgpu::CommandEncoder,
    ) {
        let mut spectrum = [0.0; SPECTRUM_SIZE];
        spectrum[..SPECTRUM_SIZE].clone_from_slice(&self.spectrum[..SPECTRUM_SIZE]);
        self.spectrum_texture
            .upload_data(device, encoder, bytemuck::bytes_of(&spectrum));
    }
}
