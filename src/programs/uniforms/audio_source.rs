use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

pub const FRAME_SIZE: usize = 512;

pub fn lerp(prev: f32, next: f32, smoothing: f32) -> f32 {
    smoothing * prev + (1.0 - smoothing) * next
}

#[derive(Debug, Clone)]
pub enum AudioMessage {
    Close,
    Data(Vec<f32>),
    Error(String),
}

#[derive(Debug, Clone)]
pub struct Subscriber {
    name: String,
    channel: Sender<AudioMessage>,
}

#[derive(Debug, Clone)]
pub enum ControlMessage {
    Close,
    Subscribe(Subscriber),
    Unsubscribe(String),
}

pub type Subscribers = HashMap<String, Sender<AudioMessage>>;

pub struct AudioSource {
    pub error: Option<String>,
    pub sample_rate: f32,

    control_channel_tx: Option<Sender<ControlMessage>>,
    control_thread: Option<std::thread::JoinHandle<()>>,
    error_channel_rx: Option<Receiver<String>>,
    running: bool,
    stream: Option<cpal::Stream>,
    subscriber_count: i32,
}

impl AudioSource {
    pub fn new() -> Self {
        Self {
            control_channel_tx: None,
            control_thread: None,
            error: None,
            error_channel_rx: None,
            sample_rate: 44100.0,
            running: false,
            stream: None,
            subscriber_count: 0,
        }
    }

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
        self.sample_rate = sample_rate as f32;

        let (control_channel_tx, control_channel_rx) = channel();
        self.control_channel_tx = Some(control_channel_tx);

        let (audio_channel_tx, audio_channel_rx) = channel();
        let audio_channel_tx2 = audio_channel_tx.clone();

        // build audio stream
        let stream_builder = audio_device.build_input_stream(
            &audio_config.config(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                audio_channel_tx
                    .send(AudioMessage::Data(data.to_vec()))
                    .unwrap();
            },
            move |err| {
                let message = format!("Error reading frame from audio stream: {:?}", err);
                audio_channel_tx2
                    .send(AudioMessage::Error(message))
                    .unwrap();
            },
        );

        let (error_channel_tx, error_channel_rx) = channel();
        self.error_channel_rx = Some(error_channel_rx);

        self.control_thread = Some(thread::spawn(move || {
            let mut subscribers = Subscribers::new();

            'outer: loop {
                // forward audio messages to subscribers
                if let Ok(msg) = audio_channel_rx.try_recv() {
                    subscribers
                        .values()
                        .into_iter()
                        .for_each(|s| s.send(msg.clone()).unwrap());

                    // break if error
                    if let AudioMessage::Error(error) = msg {
                        error_channel_tx.send(error).unwrap();
                        break 'outer;
                    }
                }

                // receive message from control thread
                if let Ok(msg) = control_channel_rx.try_recv() {
                    match msg {
                        ControlMessage::Close => {
                            subscribers
                                .values()
                                .into_iter()
                                .for_each(|s| s.send(AudioMessage::Close).unwrap());
                            break 'outer;
                        }
                        ControlMessage::Subscribe(Subscriber { name, channel }) => {
                            subscribers.insert(name, channel);
                        }
                        ControlMessage::Unsubscribe(name) => {
                            subscribers.remove(&name);
                        }
                    }
                }
            }
        }));

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
        self.running = true;
        true
    }

    pub fn send_control_message(&mut self, msg: ControlMessage) {
        if let Some(control_channel) = &self.control_channel_tx {
            control_channel.send(msg).unwrap();
        }
    }

    pub fn end_session(&mut self) {
        if !self.running {
            return;
        }

        self.error = None;

        // stop the stream
        if let Some(stream) = self.stream.as_ref() {
            stream.pause().ok();
        }

        self.send_control_message(ControlMessage::Close);
        self.running = false;
    }

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
    }

    pub fn subscribe(&mut self, name: String, channel: Sender<AudioMessage>) {
        if !self.running {
            self.start_session();
        }

        self.send_control_message(ControlMessage::Subscribe(Subscriber { name, channel }));
        self.subscriber_count += 1;
    }

    pub fn unsubscribe(&mut self, name: String) {
        if !self.running {
            return;
        }

        self.send_control_message(ControlMessage::Unsubscribe(name));
        self.subscriber_count -= 1;
        if self.subscriber_count == 0 {
            self.end_session();
        }
    }
}
