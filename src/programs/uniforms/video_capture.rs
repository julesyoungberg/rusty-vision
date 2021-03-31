use nannou::image;
use nannou::prelude::*;
use opencv::prelude::*;
use ringbuf::{Consumer, RingBuffer};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::SystemTime;
use std::{thread, time};

use crate::util;

const FRAME_RATE: f32 = 30.0;

enum Message {
    Close(()),
    Pause(()),
    Unpause(()),
}
pub struct VideoCapture {
    pub error: Option<String>,
    pub running: bool,
    pub video_size: Vector2,
    pub video_texture: wgpu::Texture,

    capture_thread: Option<std::thread::JoinHandle<()>>,
    message_channel_tx: Sender<Message>,
    error_channel_rx: Receiver<String>,
    frame_data: Vec<u8>,
    video_consumer: Consumer<Vec<Vec<opencv::core::Vec3b>>>,
}

impl VideoCapture {
    pub fn new(device: &wgpu::Device, mut capture: opencv::videoio::VideoCapture) -> Self {
        capture.set(opencv::videoio::CAP_PROP_BUFFERSIZE, 1.0).ok();
        // save size
        let width = capture.get(opencv::videoio::CAP_PROP_FRAME_WIDTH).unwrap();
        let height = capture.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT).unwrap();
        let video_size = pt2(width as f32, height as f32);

        // create video texture
        let video_texture = util::create_texture(
            device,
            [width as u32, height as u32],
            wgpu::TextureFormat::Rgba8Uint,
        );
        let frame_data = vec![0; (width * height * 3.0) as usize];

        // setup ring buffer
        let video_ring_buffer = RingBuffer::<Vec<Vec<opencv::core::Vec3b>>>::new(2);
        let (mut video_producer, video_consumer) = video_ring_buffer.split();

        // setup communication channels
        let (error_channel_tx, error_channel_rx) = channel();
        let (message_channel_tx, message_channel_rx) = channel();

        // thread for reading from the capture
        let capture_thread = thread::spawn(move || {
            let clock = SystemTime::now();
            let mut prev = 0.0;
            let frame_dur = 1.0 / FRAME_RATE;

            loop {
                // read from camera
                let mut frame = opencv::core::Mat::default().unwrap();
                match capture.read(&mut frame) {
                    Ok(success) => {
                        if !success {
                            println!("No video frame available");
                            capture
                                .set(opencv::videoio::CAP_PROP_POS_FRAMES, 0.0)
                                .unwrap();
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
                video_producer.push(data).ok();

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
                                if let Message::Unpause(()) = message {
                                    break;
                                }
                            }
                        }
                        Message::Unpause(()) => (),
                    }
                }

                // TODO: wait the correct amount of time to delay
                // let elapsed = clock.elapsed().unwrap().as_secs_f32();
                // let diff = elapsed - prev;
                // if diff < frame_dur {
                //     thread::sleep(time::Duration::from_secs((frame_dur - diff) as u32);
                // }
                // prev = clock.elapsed().unwrap().as_secs_f32();
            }
        });

        Self {
            capture_thread: Some(capture_thread),
            message_channel_tx,
            error: None,
            error_channel_rx,
            frame_data,
            running: true,
            video_consumer,
            video_size,
            video_texture,
        }
    }

    pub fn end_session(&mut self) {
        if !self.running {
            return;
        }

        self.message_channel_tx.send(Message::Close(())).ok();
        if let Some(handle) = self.capture_thread.take() {
            handle.join().ok();
        }

        self.running = false;
    }

    pub fn update(&mut self) {
        if !self.running {
            return;
        }

        // check the error channel for errors
        if let Ok(err) = self.error_channel_rx.try_recv() {
            println!("Webcam error: {:?}", err);
            self.error = Some(err);
            self.end_session();
            return;
        }

        let popped = self.video_consumer.pop();
        if let Some(d) = popped {
            self.frame_data = d
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|pixel| pixel.iter().copied().collect::<Vec<u8>>())
                        .flatten()
                        .collect::<Vec<u8>>()
                })
                .flatten()
                .collect::<Vec<u8>>();
        }
    }

    pub fn update_texture(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        if !self.running {
            return;
        }

        let width = self.video_size.x as u32;
        let height = self.video_size.y as u32;

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
        self.video_texture
            .upload_data(device, encoder, &flat_samples.as_slice());
    }

    pub fn pause(&mut self) {
        self.message_channel_tx.send(Message::Pause(())).ok();
    }

    pub fn unpause(&mut self) {
        self.message_channel_tx.send(Message::Unpause(())).ok();
    }
}
