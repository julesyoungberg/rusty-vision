use nannou::image;
use nannou::prelude::*;
use opencv::prelude::*;
use ringbuf::{Consumer, RingBuffer};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time;

use crate::programs::config;
use crate::programs::uniforms::base::Bufferable;

enum Message {
    Close(()),
    Pause(()),
    Unpause(()),
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub video_size: Vector2,
}

pub struct WebcamUniforms {
    pub updated: bool,

    capture_thread: Option<std::thread::JoinHandle<()>>,
    message_channel_tx: Option<Sender<Message>>,
    data: Data,
    error_channel_rx: Option<Receiver<String>>,
    frame_data: Vec<u8>,
    running: bool,
    video_consumer: Option<Consumer<Vec<u8>>>,
    video_texture: Option<wgpu::Texture>,
}

impl Bufferable<Data> for WebcamUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }

    fn textures(&self) -> Vec<&wgpu::Texture> {
        match &self.video_texture {
            Some(t) => vec![&t],
            None => vec![],
        }
    }
}

impl WebcamUniforms {
    pub fn new() -> Self {
        Self {
            capture_thread: None,
            message_channel_tx: None,
            data: Data {
                video_size: pt2(0.0, 0.0),
            },
            error_channel_rx: None,
            frame_data: vec![],
            running: false,
            updated: false,
            video_consumer: None,
            video_texture: None,
        }
    }

    pub fn set_defaults(
        &mut self,
        device: &wgpu::Device,
        _default: &Option<config::ProgramDefaults>,
    ) {
        self.running = self.start_session(device);
    }

    /// Starts a webcam session.
    /// Spawns a thread to consumer webcam data with OpenCV.
    pub fn start_session(&mut self, device: &wgpu::Device) -> bool {
        if self.running {
            return true;
        }

        let (error_channel_tx, error_channel_rx) = channel();
        self.error_channel_rx = Some(error_channel_rx);

        let (message_channel_tx, message_channel_rx) = channel();
        self.message_channel_tx = Some(message_channel_tx);

        let mut capture = opencv::videoio::VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();

        let width = capture.get(opencv::videoio::CAP_PROP_FRAME_WIDTH).unwrap();
        let height = capture.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT).unwrap();

        self.data.video_size = pt2(width as f32, height as f32);

        self.video_texture = Some(
            wgpu::TextureBuilder::new()
                .size([width as u32, height as u32])
                .usage(wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED)
                .format(wgpu::TextureFormat::Rgba8Uint)
                .build(device),
        );
        self.frame_data = vec![0 as u8; (width * height * 3.0) as usize];

        let video_ring_buffer = RingBuffer::<Vec<u8>>::new(2);
        let (mut video_producer, video_consumer) = video_ring_buffer.split();
        video_producer
            .push(vec![0 as u8; (width * height * 3.0) as usize])
            .unwrap();
        self.video_consumer = Some(video_consumer);

        self.capture_thread = Some(thread::spawn(move || loop {
            // read from camera
            let mut frame = opencv::core::Mat::default().unwrap();
            match capture.read(&mut frame) {
                Ok(success) => {
                    if !success {
                        println!("No video frame available");
                        continue;
                    }
                }
                Err(e) => {
                    println!("Error capturing video frame: {:?}", e);
                    error_channel_tx.send(e.to_string()).unwrap();
                    break;
                }
            }

            // get usable data
            let data: Vec<Vec<opencv::core::Vec3b>> = frame.to_vec_2d().unwrap();

            let img_data = data
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|pixel| pixel.iter().map(|v| *v).collect::<Vec<u8>>())
                        .flatten()
                        .collect::<Vec<u8>>()
                })
                .flatten()
                .collect::<Vec<u8>>();

            video_producer.push(img_data).unwrap();

            if let Ok(msg) = message_channel_rx.try_recv() {
                match msg {
                    Message::Close(()) => {
                        // break from the outer loop
                        println!("Closing capture thread");
                        break;
                    }
                    Message::Pause(()) => {
                        // the stream has been paused, block it is unpaused
                        for message in message_channel_rx.iter() {
                            match message {
                                Message::Unpause(()) => break, // break from this inner loop
                                _ => (),                       // continue waiting
                            }
                        }
                    }
                    Message::Unpause(()) => (),
                }
            }

            thread::sleep(time::Duration::from_millis(50));
        }));

        self.updated = true;

        return true;
    }

    pub fn end_session(&mut self) {
        if !self.running {
            return;
        }

        if let Some(message_channel) = self.message_channel_tx.take() {
            message_channel.send(Message::Close(())).unwrap();
        }

        if let Some(handle) = self.capture_thread.take() {
            handle.join().unwrap();
        }

        self.running = false;
    }

    pub fn update(&mut self) {
        if !self.running {
            return;
        }

        // check the error channel for errors
        if let Ok(err) = self.error_channel_rx.as_ref().unwrap().try_recv() {
            println!("Webcam error: {:?}", err);
            self.end_session();
            return;
        }

        match self.video_consumer.take() {
            Some(mut c) => {
                let popped = c.pop();
                self.video_consumer = Some(c);
                match popped {
                    Some(d) => self.frame_data = d,
                    None => (),
                };
            }
            None => (),
        };
    }

    pub fn update_texture(
        &self,
        device: &wgpu::Device,
        encoder: &mut nannou::wgpu::CommandEncoder,
    ) {
        if let Some(video_texture) = &self.video_texture {
            let width = self.data.video_size.x as u32;
            let height = self.data.video_size.y as u32;

            let image = image::ImageBuffer::from_fn(width, height, |x, y| {
                let index = (((height - y - 1) * width + (width - x - 1)) * 3) as usize;
                // convert from BGR to RGB
                image::Rgba([
                    self.frame_data[index + 2],
                    self.frame_data[index + 1],
                    self.frame_data[index],
                    std::u8::MAX,
                ])
            });

            let flat_samples = image.as_flat_samples();
            video_texture.upload_data(device, encoder, &flat_samples.as_slice());
        }
    }

    pub fn pause(&mut self) {
        if let Some(message_channel) = self.message_channel_tx.take() {
            message_channel.send(Message::Pause(())).unwrap();
            self.message_channel_tx = Some(message_channel);
        }
    }

    pub fn unpause(&mut self) {
        if let Some(message_channel) = self.message_channel_tx.take() {
            message_channel.send(Message::Unpause(())).unwrap();
            self.message_channel_tx = Some(message_channel);
        }
    }
}
