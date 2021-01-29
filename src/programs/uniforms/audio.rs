use cpal;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use nannou::prelude::*;
use ringbuf::{Consumer, RingBuffer};
use serde_json;
use serde_json::{json, Value};
use std::string::ToString;
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time;
use websocket::client::ClientBuilder;
use websocket::OwnedMessage;

use crate::programs::uniforms::base::Bufferable;

const CONNECTION: &'static str = "ws://127.0.0.1:9002";

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
    pub tristimulus: [f32; 3],
}

pub struct AudioUniforms {
    pub data: Data,
    pub error: Option<String>,

    consumer: Option<Consumer<serde_json::Value>>,
    error_channel: Option<Receiver<String>>,
    recv_thread: Option<std::thread::JoinHandle<()>>,
    running: bool,
    smoothing: f32,
    stream: Option<cpal::Stream>,
}

impl Bufferable for AudioUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }

    fn set_program_defaults(&mut self, _selected: usize) {
        self.start_session();
    }
}

impl AudioUniforms {
    pub fn new() -> Self {
        Self {
            consumer: None,
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
                tristimulus: [0.0, 0.0, 0.0],
            },
            error: None,
            error_channel: None,
            recv_thread: None,
            running: false,
            smoothing: 0.6,
            stream: None,
        }
    }

    /**
     * Initialize the MIR session with the mirlin server
     * - setup audio listener
     * - establish subscription with the server
     */
    pub fn start_session(&mut self) -> bool {
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

        let (mut receiver, mut sender) = match client.split() {
            Ok(t) => t,
            Err(e) => {
                self.error = Some(format!(
                    "Error splitting audio client into sender & receiver: {:?}",
                    e
                ));
                return false;
            }
        };

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
                    // "mfcc",
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

        println!("confirmation message: {:?}", confirmation_msg);

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

        let (tx, rx) = channel();
        self.error_channel = Some(rx);
        let tx_1 = tx.clone();
        let tx_2 = tx.clone();

        // build audio stream
        let stream_builder = audio_device.build_input_stream(
            &audio_config.config(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let frame_message = json!({
                    "type": "audio_frame",
                    "payload": data,
                });

                // send frame to server
                let message = OwnedMessage::Text(frame_message.to_string());
                match sender.send_message(&message) {
                    Ok(()) => (),
                    Err(e) => {
                        tx.send(format!("Error sending frame: {:?}", e)).unwrap();
                    }
                }
            },
            move |err| {
                tx_1.send(format!("Error reading frame from audio stream: {:?}", err))
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

        // create a ring buffer for server responses (features)
        let ring_buffer = RingBuffer::<serde_json::Value>::new(2);
        let (mut producer, consumer) = ring_buffer.split();
        producer.push(json!(null)).unwrap();
        self.consumer = Some(consumer);

        // listen for messages from server, push to ring buffer
        self.recv_thread = Some(thread::spawn(move || {
            for raw in receiver.incoming_messages() {
                let message = match raw {
                    Ok(m) => m,
                    Err(e) => {
                        tx_2.send(format!("Error receiving message from mirlin: {:?}", e))
                            .unwrap();
                        return;
                    }
                };
                let value: Value = match message {
                    OwnedMessage::Text(json_string) => match serde_json::from_str(&json_string) {
                        Ok(v) => v,
                        Err(e) => {
                            tx_2.send(format!("Error parsing message from mirlin: {:?}", e))
                                .unwrap();
                            return;
                        }
                    },
                    _ => {
                        tx_2.send(format!(
                            "Received unexpected message from mirlin: {:?}",
                            message
                        ))
                        .unwrap();
                        return;
                    }
                };
                producer.push(value).ok();
            }
        }));

        self.running = true;

        return true;
    }

    /**
     * Ends a session by stopping the stream, disconnecting from the server,
     * and any other clean up
     */
    pub fn end_session(&mut self) {
        if !self.running {
            return;
        }

        if let Some(stream) = self.stream.as_ref() {
            match stream.pause() {
                _ => (),
            };
        }

        // TODO: disconnect from mirlin server

        if let Some(handle) = self.recv_thread.take() {
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

    /**
     * Update data based on recently received features and handle any errors
     */
    pub fn update(&mut self) {
        if !self.running {
            return;
        }

        // check the errro channel for errors
        if let Ok(err) = self
            .error_channel
            .as_ref()
            .unwrap()
            .recv_timeout(time::Duration::from_millis(0))
        {
            println!("Audio error: {:?}", err);
            self.end_session();
            return;
        }

        let current = match self.consumer.take() {
            Some(mut c) => match c.pop() {
                Some(v) => v,
                None => return,
            },
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

        println!("features: {:?}", features);

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

        let pitch = self.unwrap_feature(features.get("pitch.mean"));
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
        for i in 0..3 {
            self.data.tristimulus[i] = self.lerp(
                self.data.tristimulus[i],
                tristimulus[i].as_f64().unwrap() as f32,
            );
        }
    }
}
