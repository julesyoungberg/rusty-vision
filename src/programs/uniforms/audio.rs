use bytemuck;
use cpal;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use nannou::prelude::*;
use ringbuf::{Consumer, RingBuffer};
use rustfft::{num_complex::Complex, FftPlanner};
use serde_json;
use serde_json::{json, Value};
use std::string::ToString;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use websocket::client::ClientBuilder;
use websocket::OwnedMessage;

use crate::programs::config;
use crate::programs::uniforms::base::Bufferable;

const CONNECTION: &'static str = "ws://127.0.0.1:9002";

const NUM_MFCCS: usize = 12;
const HOP_SIZE: usize = 512;
const WINDOW_SIZE: usize = 1024;
const SPECTRUM_SIZE: usize = 16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub dissonance: f32,
    pub energy: f32,
    pub loudness: f32,
    pub noisiness: f32,
    pub onset: f32,
    pub pitch: f32,
    pub rms: f32,
    pub spectral_centroid: f32,
    pub spectral_complexity: f32,
    pub spectral_contrast: f32,
    pub tristimulus1: f32,
    pub tristimulus2: f32,
    pub tristimulus3: f32,
}

pub struct AudioUniforms {
    pub data: Data,
    pub error: Option<String>,
    pub mfcc_texture: wgpu::Texture,
    pub running: bool,
    pub smoothing: f32,
    pub spectrum_texture: wgpu::Texture,

    error_channel_rx: Option<Receiver<String>>,
    feature_consumer: Option<Consumer<serde_json::Value>>,
    fft_close_channel_tx: Option<Sender<OwnedMessage>>,
    fft_thread: Option<std::thread::JoinHandle<()>>,
    mfccs: [f32; NUM_MFCCS],
    recv_thread: Option<std::thread::JoinHandle<()>>,
    send_close_channel_tx: Option<Sender<OwnedMessage>>,
    send_thread: Option<std::thread::JoinHandle<()>>,
    spectrum: Vec<f32>,
    spectrum_consumer: Option<Consumer<Vec<f32>>>,
    stream: Option<cpal::Stream>,
}

impl Bufferable<Data> for AudioUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }

    fn textures(&self) -> Vec<&wgpu::Texture> {
        vec![&self.mfcc_texture, &self.spectrum_texture]
    }
}

impl AudioUniforms {
    pub fn new(device: &wgpu::Device) -> Self {
        let mfcc_texture = wgpu::TextureBuilder::new()
            .size([NUM_MFCCS as u32, 1])
            .format(wgpu::TextureFormat::R32Float)
            .usage(wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED)
            .build(device);

        let spectrum_texture = wgpu::TextureBuilder::new()
            .size([SPECTRUM_SIZE as u32, 1])
            .format(wgpu::TextureFormat::R32Float)
            .usage(wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED)
            .build(device);

        Self {
            data: Data {
                dissonance: 0.0,
                energy: 0.0,
                loudness: 0.0,
                noisiness: 0.0,
                onset: 0.0,
                pitch: 0.0,
                rms: 0.0,
                spectral_centroid: 0.0,
                spectral_complexity: 0.0,
                spectral_contrast: 0.0,
                tristimulus1: 0.0,
                tristimulus2: 0.0,
                tristimulus3: 0.0,
            },
            error: None,
            error_channel_rx: None,
            feature_consumer: None,
            fft_close_channel_tx: None,
            fft_thread: None,
            mfccs: [0.0; NUM_MFCCS],
            mfcc_texture,
            recv_thread: None,
            running: false,
            send_close_channel_tx: None,
            send_thread: None,
            smoothing: 0.9,
            spectrum: vec![0.0; SPECTRUM_SIZE],
            spectrum_consumer: None,
            spectrum_texture,
            stream: None,
        }
    }

    pub fn set_defaults(&mut self, defaults: &Option<config::ProgramDefaults>) {
        self.smoothing = 0.5;

        if let Some(cnfg) = defaults {
            if let Some(smoothing) = cnfg.audio_feature_smoothing {
                self.smoothing = smoothing;
            }
        }

        self.running = self.start_session();
    }

    /// Initialize the MIR session with the mirlin server
    /// - setup audio listener
    /// - establish subscription with the server
    pub fn start_session(&mut self) -> bool {
        if self.running {
            return true;
        }

        // get default audio input device
        let audio_device = match cpal::default_host().default_input_device() {
            Some(device) => device,
            None => {
                self.error = Some(String::from("Unable to connect to default audio device"));
                return false;
            }
        };

        // find supported config
        let supported_configs = match audio_device.supported_input_configs() {
            Ok(mut configs) => match configs.next() {
                Some(config) => config,
                None => {
                    self.error = Some(String::from("No audio configuration available"));
                    return false;
                }
            },
            Err(e) => {
                self.error = Some(format!("Error configuring audio input: {:?}", e));
                return false;
            }
        };

        let audio_config = supported_configs.with_max_sample_rate();
        let cpal::SampleRate(sample_rate) = audio_config.sample_rate();

        // create websocket client
        let client_builder = match ClientBuilder::new(CONNECTION) {
            Ok(client) => client,
            Err(e) => {
                self.error = Some(format!("Error building audio websocket client: {:?}", e));
                return false;
            }
        };

        // connect to mirlin
        let client = match client_builder
            .add_protocol("rust-websocket")
            .connect_insecure()
        {
            Ok(client) => client,
            Err(e) => {
                self.error = Some(format!("Error connecting to the mirlin server: {:?}", e));
                return false;
            }
        };

        let (mut receiver, mut sender) = client.split().unwrap();

        // build subscription request
        let session_request = json!({
            "type": "session_request",
            "payload": {
                "features": [
                    "centroid",
                    "dissonance",
                    "energy",
                    // "key",
                    "loudness",
                    "mfcc",
                    "noisiness",
                    "onset",
                    "pitch",
                    "rms",
                    "spectral_complexity",
                    "spectral_contrast",
                    "tristimulus",
                ],
                "sample_rate": sample_rate,
                "hop_size": 512 as u32, // happens to be cpal's buffer size
                "memory": 4 as u32, // rember 4 frames including current
            }
        });

        // request subscription
        let request_message = OwnedMessage::Text(session_request.to_string());
        match sender.send_message(&request_message) {
            Ok(()) => (),
            Err(e) => {
                self.error = Some(format!(
                    "Error requesting audio subscription with mirlin: {:?}",
                    e
                ));
                return false;
            }
        };

        // wait for confirmation
        let confirmation_msg = match receiver.recv_message() {
            Ok(msg) => msg,
            Err(e) => {
                self.error = Some(format!(
                    "Error configuring audio subscription with mirlin: {:?}",
                    e
                ));
                return false;
            }
        };

        match confirmation_msg {
            OwnedMessage::Text(_json_string) => (),
            _ => {
                self.error = Some(format!(
                    "Received invalid confirmation from mirlin server: {:?}",
                    confirmation_msg
                ));
                return false;
            }
        };

        let (error_channel_tx, error_channel_rx) = channel();
        self.error_channel_rx = Some(error_channel_rx);
        let error_channel_tx_1 = error_channel_tx.clone();

        let (audio_channel_tx, audio_channel_rx) = channel();
        let (send_close_channel_tx, send_close_channel_rx) = channel();
        self.send_close_channel_tx = Some(send_close_channel_tx);

        let (fft_input_channel_tx, fft_input_channel_rx) = channel();

        // build audio stream
        let stream_builder = audio_device.build_input_stream(
            &audio_config.config(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                audio_channel_tx.send(data.to_vec()).unwrap();
                fft_input_channel_tx.send(data.to_vec()).unwrap();
            },
            move |err| {
                error_channel_tx
                    .send(format!("Error reading frame from audio stream: {:?}", err))
                    .unwrap();
            },
        );

        // create stream
        let stream = match stream_builder {
            Ok(s) => s,
            Err(e) => {
                self.error = Some(format!("Error creating audio stream: {:?}", e));
                return false;
            }
        };

        // start stream
        match stream.play() {
            Ok(()) => (),
            Err(e) => {
                self.error = Some(format!("Error starting audio stream: {:?}", e));
                return false;
            }
        };

        self.stream = Some(stream);

        // sender thread
        // forward audio from the audio thread to mirlin
        // check the close channel for messages to end the session
        self.send_thread = Some(thread::spawn(move || {
            'sender: loop {
                // check the audio channel for data, forward to mirlin if available
                if let Ok(data) = audio_channel_rx.try_recv() {
                    let frame_message = json!({
                        "type": "audio_frame",
                        "payload": &data[..],
                    });
                    let message = OwnedMessage::Text(frame_message.to_string());
                    match sender.send_message(&message) {
                        Ok(()) => (),
                        Err(e) => {
                            println!("Error sending frame: {:?}", e);
                            break 'sender;
                        }
                    }
                }

                // check the close channel, break if needed
                if let Ok(m) = send_close_channel_rx.try_recv() {
                    if let OwnedMessage::Close(_) = m {
                        break 'sender;
                    }
                }
            }

            // close the connection and end the session
            sender.shutdown_all().unwrap();
        }));

        // create a ring buffer for server responses (features)
        let feature_ring_buffer = RingBuffer::<serde_json::Value>::new(2);
        let (mut feature_producer, feature_consumer) = feature_ring_buffer.split();
        feature_producer.push(json!(null)).unwrap();
        self.feature_consumer = Some(feature_consumer);

        // listen for messages from server, push to ring buffer
        self.recv_thread = Some(thread::spawn(move || {
            for raw in receiver.incoming_messages() {
                let message = match raw {
                    Ok(m) => m,
                    Err(e) => {
                        error_channel_tx_1
                            .send(format!("Error receiving message from mirlin: {:?}", e))
                            .unwrap();
                        return;
                    }
                };
                let value: Value = match message {
                    OwnedMessage::Text(json_string) => match serde_json::from_str(&json_string) {
                        Ok(v) => v,
                        Err(e) => {
                            error_channel_tx_1
                                .send(format!("Error parsing message from mirlin: {:?}", e))
                                .unwrap();
                            return;
                        }
                    },
                    _ => {
                        error_channel_tx_1
                            .send(format!(
                                "Received unexpected message from mirlin: {:?}",
                                message
                            ))
                            .unwrap();
                        return;
                    }
                };
                feature_producer.push(value).ok();
            }
        }));

        // setup the FFT
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(WINDOW_SIZE);
        let hanning_window = apodize::hanning_iter(WINDOW_SIZE).collect::<Vec<f64>>();

        // create a ring buffer for spectrum results
        let spectrum_ring_buffer = RingBuffer::<Vec<f32>>::new(2);
        let (mut spectrum_producer, spectrum_consumer) = spectrum_ring_buffer.split();
        spectrum_producer.push(vec![0.0; SPECTRUM_SIZE]).unwrap();
        self.spectrum_consumer = Some(spectrum_consumer);
        let spec_group_size = (WINDOW_SIZE / 2) / SPECTRUM_SIZE;

        let (fft_close_channel_tx, fft_close_channel_rx) = channel();
        self.fft_close_channel_tx = Some(fft_close_channel_tx);

        // create the fft thread
        self.fft_thread = Some(thread::spawn(move || {
            let mut frames = vec![];
            frames.push(vec![0.0; HOP_SIZE]);
            frames.push(vec![0.0; HOP_SIZE]);

            loop {
                if let Ok(frame) = fft_input_channel_rx.try_recv() {
                    // add new frame to memory and build the window
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

                    spectrum_producer.push(reduced_spectrum).ok();
                }

                // check the close channel, break if needed
                if let Ok(m) = fft_close_channel_rx.try_recv() {
                    if let OwnedMessage::Close(_) = m {
                        break;
                    }
                }
            }
        }));

        return true;
    }

    /// Ends a session by stopping the stream, disconnecting from the server,
    /// and any other clean up
    pub fn end_session(&mut self) {
        if !self.running {
            return;
        }

        // stop the stream
        if let Some(stream) = self.stream.as_ref() {
            match stream.pause() {
                _ => (),
            };
        }

        // send a message to the close channel to stop the sender thread
        if let Some(close_channel) = self.send_close_channel_tx.take() {
            close_channel.send(OwnedMessage::Close(None)).unwrap();
        }

        // send a message to the close channel to stop the fft thread
        if let Some(close_channel) = self.fft_close_channel_tx.take() {
            close_channel.send(OwnedMessage::Close(None)).unwrap();
        }

        // join the sender thread
        if let Some(handle) = self.send_thread.take() {
            handle.join().unwrap();
        }

        // join the receiver thread
        if let Some(handle) = self.recv_thread.take() {
            handle.join().unwrap();
        }

        // join the fft thread
        if let Some(handle) = self.fft_thread.take() {
            handle.join().unwrap();
        }

        self.running = false;
    }

    fn unwrap_feature(&self, v: Option<&Value>) -> f32 {
        v.unwrap().as_array().unwrap()[0].as_f64().unwrap() as f32
    }

    fn lerp(&self, prev: f32, next: f32) -> f32 {
        self.smoothing * prev + (1.0 - self.smoothing) * next
    }

    /// Update data based on recently received features and handle any errors
    pub fn update(&mut self) {
        if !self.running {
            return;
        }

        // check the error channel for errors
        if let Ok(err) = self.error_channel_rx.as_ref().unwrap().try_recv() {
            println!("Audio error: {:?}", err);
            self.end_session();
            return;
        }

        // this is kind of gross
        // take the consumer out of the option to mutate it by popping
        // then put it back in the option for next time
        match self.spectrum_consumer.take() {
            Some(mut c) => {
                let popped = c.pop();
                self.spectrum_consumer = Some(c);
                match popped {
                    Some(s) => {
                        for i in 0..SPECTRUM_SIZE {
                            self.spectrum[i] = self.lerp(self.spectrum[i], s[i]);
                        }
                    }
                    None => (),
                };
            }
            None => (),
        };

        // and again for the features
        let current = match self.feature_consumer.take() {
            Some(mut c) => {
                let popped = c.pop();
                self.feature_consumer = Some(c);
                match popped {
                    Some(v) => v,
                    None => return,
                }
            }
            None => return,
        };

        let payload = match current.get("payload") {
            Some(p) => p,
            None => return,
        };

        let features = match payload.get("features") {
            Some(f) => f,
            None => return,
        };

        if let Some(onset) = features.get("onset").unwrap().as_array() {
            self.data.onset = onset[0].as_f64().unwrap() as f32;
        }

        let dissonance = self.unwrap_feature(features.get("dissonance.mean"));
        self.data.dissonance = self.lerp(self.data.dissonance, dissonance);

        let energy = self.unwrap_feature(features.get("energy.mean"));
        self.data.energy = self.lerp(self.data.energy, energy);

        let loudness = self.unwrap_feature(features.get("loudness.mean"));
        self.data.loudness = self.lerp(self.data.loudness, loudness);

        let noisiness = self.unwrap_feature(features.get("noisiness.mean"));
        self.data.noisiness = self.lerp(self.data.noisiness, noisiness);

        let pitch = self.unwrap_feature(features.get("f0.mean"));
        self.data.pitch = self.lerp(self.data.pitch, pitch);

        let rms = self.unwrap_feature(features.get("rms.mean"));
        self.data.rms = self.lerp(self.data.rms, rms);

        let centroid = self.unwrap_feature(features.get("centroid.mean"));
        self.data.spectral_centroid = self.lerp(self.data.spectral_centroid, centroid);

        let spectral_complexity = self.unwrap_feature(features.get("spectral_complexity.mean"));
        self.data.spectral_complexity =
            self.lerp(self.data.spectral_complexity, spectral_complexity);

        let spectral_contrast = self.unwrap_feature(features.get("spectral_contrast.mean"));
        self.data.spectral_contrast = self.lerp(self.data.spectral_contrast, spectral_contrast);

        let tristimulus = features
            .get("tristimulus.mean")
            .unwrap()
            .as_array()
            .unwrap();
        self.data.tristimulus1 = self.lerp(
            self.data.tristimulus1,
            tristimulus[0].as_f64().unwrap() as f32,
        );
        self.data.tristimulus2 = self.lerp(
            self.data.tristimulus2,
            tristimulus[1].as_f64().unwrap() as f32,
        );
        self.data.tristimulus3 = self.lerp(
            self.data.tristimulus3,
            tristimulus[2].as_f64().unwrap() as f32,
        );

        let mfccs = features.get("mfcc.mean").unwrap().as_array().unwrap();
        for i in 0..NUM_MFCCS {
            self.mfccs[i] = self.lerp(
                self.mfccs[i],
                mfccs[i + 1].as_f64().unwrap().max(0.0) as f32,
            );
        }
    }

    /// Update GPU textures with new data
    pub fn update_textures(
        &self,
        device: &wgpu::Device,
        encoder: &mut nannou::wgpu::CommandEncoder,
    ) {
        self.mfcc_texture
            .upload_data(device, encoder, bytemuck::bytes_of(&self.mfccs));

        let mut spectrum = [0.0; SPECTRUM_SIZE];
        for i in 0..SPECTRUM_SIZE {
            spectrum[i] = self.spectrum[i];
        }
        self.spectrum_texture
            .upload_data(device, encoder, bytemuck::bytes_of(&spectrum));
    }
}
