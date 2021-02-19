use cpal;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc::{channel, Receiver, Sender};

pub const FRAME_SIZE: usize = 512;

pub fn lerp(prev: f32, next: f32, smoothing: f32) -> f32 {
    smoothing * prev + (1.0 - smoothing) * next
}

pub enum AudioMessage {
    Data(Vec<f32>),
    Close(()),
}

pub struct AudioSource {
    pub error: Option<String>,
    pub sample_rate: f32,

    audio_channels: Vec<Sender<AudioMessage>>,
    error_channels: Vec<Sender<String>>,
    error_channel_rx: Option<Receiver<String>>,
    stream: Option<cpal::Stream>,
}

impl AudioSource {
    pub fn new() -> Self {
        Self {
            audio_channels: vec![],
            error: None,
            error_channels: vec![],
            error_channel_rx: None,
            sample_rate: 44100.0,
            stream: None,
        }
    }

    pub fn start_session(
        &mut self,
        audio_channels: Vec<Sender<AudioMessage>>,
        error_channels: Vec<Sender<String>>,
    ) -> bool {
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
        self.sample_rate = sample_rate as f32;

        self.audio_channels = audio_channels.iter().map(|c| c.clone()).collect();
        self.error_channels = error_channels.iter().map(|c| c.clone()).collect();

        let (error_channel_tx, error_channel_rx) = channel();
        self.error_channel_rx = Some(error_channel_rx);

        // build audio stream
        let stream_builder = audio_device.build_input_stream(
            &audio_config.config(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                audio_channels.iter().for_each(|c| {
                    c.send(AudioMessage::Data(data.to_vec())).unwrap();
                });
            },
            move |err| {
                let message = format!("Error reading frame from audio stream: {:?}", err);
                error_channel_tx.send(message.clone()).unwrap();
                error_channels.iter().for_each(|c| {
                    c.send(message.clone()).unwrap();
                });
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

        return true;
    }

    pub fn end_session(&mut self) {
        self.error = None;

        // stop the stream
        if let Some(stream) = self.stream.as_ref() {
            match stream.pause() {
                _ => (),
            };
        }

        self.audio_channels.iter().for_each(|c| {
            c.send(AudioMessage::Close(())).ok();
        });

        self.audio_channels = vec![];
    }

    pub fn update(&mut self) {
        // check the error channel for errors
        if let Ok(err) = self.error_channel_rx.as_ref().unwrap().try_recv() {
            println!("Audio error: {:?}", err);
            self.end_session();
            return;
        }
    }
}
