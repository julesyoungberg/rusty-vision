use nannou::prelude::*;
use opencv::prelude::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use crate::programs::config;
use crate::programs::uniforms::base::Bufferable;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub image_size: Vector2,
}

pub struct WebcamUniforms {
    capture_thread: Option<std::thread::JoinHandle<()>>,
    close_channel_tx: Option<Sender<()>>,
    data: Data,
    error_channel_rx: Option<Receiver<String>>,
    running: bool,
}

impl Bufferable<Data> for WebcamUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }

    fn textures(&self) -> Option<Vec<&wgpu::Texture>> {
        None
    }
}

impl WebcamUniforms {
    pub fn new(device: &wgpu::Device) -> Self {
        Self {
            capture_thread: None,
            close_channel_tx: None,
            data: Data {
                image_size: pt2(0.0, 0.0),
            },
            error_channel_rx: None,
            running: false,
        }
    }

    pub fn set_defaults(&mut self, default: &Option<config::ProgramDefaults>) {
        self.running = self.start_session();
    }

    pub fn start_session(&mut self) -> bool {
        if self.running {
            return true;
        }

        let (error_channel_tx, error_channel_rx) = channel();
        self.error_channel_rx = Some(error_channel_rx);

        let (close_channel_tx, close_channel_rx) = channel();
        self.close_channel_tx = Some(close_channel_tx);

        let mut capture = opencv::videoio::VideoCapture::default().unwrap();

        self.capture_thread = Some(thread::spawn(move || loop {
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

            println!("captured frame: {:?}", frame);

            if let Ok(()) = close_channel_rx.try_recv() {
                println!("Closing capture thread");
                break;
            }
        }));

        return true;
    }

    pub fn end_session(&mut self) {
        if !self.running {
            return;
        }

        if let Some(close_channel) = self.close_channel_tx.take() {
            close_channel.send(()).unwrap();
        }

        if let Some(handle) = self.capture_thread.take() {
            handle.join().unwrap();
        }
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
    }
}
