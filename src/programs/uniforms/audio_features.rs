use nannou::prelude::*;
use ringbuf::{Consumer, RingBuffer};
use serde_json::{json, Value};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use websocket::client::ClientBuilder;
use websocket::OwnedMessage;

use crate::programs::config;
use crate::programs::uniforms::audio_source;
use crate::programs::uniforms::base::Bufferable;
use crate::util;

const CONNECTION: &str = "ws://127.0.0.1:9002";
const NUM_MFCCS: usize = 12;

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

pub struct AudioFeaturesUniforms {
    pub audio_channel_tx: Option<Sender<audio_source::AudioMessage>>,
    pub data: Data,
    pub error: Option<String>,
    pub smoothing: f32,

    error_channel_rx: Option<Receiver<String>>,
    recv_thread: Option<std::thread::JoinHandle<()>>,
    feature_consumer: Option<Consumer<serde_json::Value>>,
    mfccs: [f32; NUM_MFCCS],
    mfcc_texture: wgpu::Texture,
    send_thread: Option<std::thread::JoinHandle<()>>,
}

impl Bufferable<Data> for AudioFeaturesUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }

    fn textures(&self) -> Vec<&wgpu::Texture> {
        vec![&self.mfcc_texture]
    }
}

impl AudioFeaturesUniforms {
    pub fn new(device: &wgpu::Device) -> Self {
        let mfcc_texture =
            util::create_texture(device, [NUM_MFCCS as u32, 1], wgpu::TextureFormat::R32Float);

        Self {
            audio_channel_tx: None,
            error_channel_rx: None,
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
            feature_consumer: None,
            mfccs: [0.0; NUM_MFCCS],
            mfcc_texture,
            recv_thread: None,
            smoothing: 0.5,
            send_thread: None,
        }
    }

    pub fn configure(&mut self, settings: &Option<config::ProgramSettings>) {
        self.smoothing = 0.5;

        if let Some(cnfg) = settings {
            if let Some(smoothing) = cnfg.audio_feature_smoothing {
                self.smoothing = smoothing;
            }
        }
    }

    pub fn start_session(&mut self, audio_source: &mut audio_source::AudioSource) -> bool {
        let (audio_channel_tx, audio_channel_rx) = channel();
        audio_source.subscribe(String::from("audio_features"), audio_channel_tx.clone());
        self.audio_channel_tx = Some(audio_channel_tx.clone());

        let (error_channel_tx, error_channel_rx) = channel();
        self.error_channel_rx = Some(error_channel_rx);

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
                "sample_rate": audio_source.sample_rate,
                "hop_size": 512, // happens to be cpal's buffer size
                "memory": 4, // rember 4 frames including current
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

        // sender thread
        // forward audio from the audio thread to mirlin
        // check the close channel for messages to end the session
        let error_channel_tx2 = error_channel_tx.clone();
        self.send_thread = Some(thread::spawn(move || {
            'sender: for message in audio_channel_rx.iter() {
                match message {
                    audio_source::AudioMessage::Data(data) => {
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
                    audio_source::AudioMessage::Close => break 'sender,
                    audio_source::AudioMessage::Error(error) => {
                        error_channel_tx2.send(error).unwrap();
                        break;
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
                        error_channel_tx
                            .send(format!("Error receiving message from mirlin: {:?}", e))
                            .unwrap();
                        return;
                    }
                };
                let value: Value = match message {
                    OwnedMessage::Text(json_string) => match serde_json::from_str(&json_string) {
                        Ok(v) => v,
                        Err(e) => {
                            error_channel_tx
                                .send(format!("Error parsing message from mirlin: {:?}", e))
                                .unwrap();
                            return;
                        }
                    },
                    _ => {
                        error_channel_tx
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

        true
    }

    pub fn end_session(&mut self) {
        self.error = None;

        if let Some(channel) = &self.audio_channel_tx {
            channel.send(audio_source::AudioMessage::Close).unwrap();
        }

        // join the sender thread
        if let Some(handle) = self.send_thread.take() {
            handle.join().unwrap();
        }

        // join the receiver thread
        if let Some(handle) = self.recv_thread.take() {
            handle.join().unwrap();
        }
    }

    fn unwrap_feature(&self, v: Option<&Value>) -> f32 {
        v.unwrap().as_array().unwrap()[0].as_f64().unwrap() as f32
    }

    fn lerp(&self, prev: f32, next: f32) -> f32 {
        audio_source::lerp(prev, next, self.smoothing)
    }

    pub fn update(&mut self) {
        // check the error channel for errors
        if let Ok(err) = self.error_channel_rx.as_ref().unwrap().try_recv() {
            println!("Audio error: {:?}", err);
            self.error = Some(err);
            self.end_session();
            return;
        }

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

    pub fn update_texture(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        self.mfcc_texture
            .upload_data(device, encoder, bytemuck::bytes_of(&self.mfccs));
    }
}
