use bytemuck;
use nannou::prelude::*;
use ringbuf::{Consumer, RingBuffer};
use std::sync::mpsc::{channel, Sender};
use std::thread;

use crate::programs::uniforms::audio_source;
use crate::programs::uniforms::base::Bufferable;

pub struct AudioUniforms {
    audio_consumer: Option<Consumer<Vec<f32>>>,
    audio_texture: wgpu::Texture,
    audio_thread: Option<std::thread::JoinHandle<()>>,
    frame: Vec<f32>,
}

impl Bufferable for AudioUniforms {
    fn textures(&self) -> Vec<&wgpu::Texture> {
        vec![&self.audio_texture]
    }
}

impl AudioUniforms {
    pub fn new(device: &wgpu::Device) -> Self {
        let audio_texture = wgpu::TextureBuilder::new()
            .size([audio_source::FRAME_SIZE as u32, 1])
            .format(wgpu::TextureFormat::R32Float)
            .usage(wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED)
            .build(device);

        Self {
            audio_consumer: None,
            audio_texture,
            audio_thread: None,
            frame: vec![],
        }
    }

    pub fn start_session(&mut self) -> Sender<audio_source::AudioMessage> {
        let (audio_channel_tx, audio_channel_rx) = channel();

        let ring_buffer = RingBuffer::<Vec<f32>>::new(2);
        let (mut producer, consumer) = ring_buffer.split();
        producer.push(vec![0.0; audio_source::FRAME_SIZE]).unwrap();
        self.audio_consumer = Some(consumer);

        self.audio_thread = Some(thread::spawn(move || {
            for msg in audio_channel_rx.iter() {
                match msg {
                    audio_source::AudioMessage::Data(frame) => producer.push(frame).ok().unwrap(),
                    audio_source::AudioMessage::Close(()) => break,
                }
            }
        }));

        audio_channel_tx
    }

    pub fn end_session(&mut self) {
        if let Some(handle) = self.audio_thread.take() {
            handle.join().unwrap();
        }
    }

    pub fn update(&mut self) {
        match self.audio_consumer.take() {
            Some(mut c) => {
                let popped = c.pop();
                self.audio_consumer = Some(c);
                match popped {
                    Some(f) => {
                        for i in 0..audio_source::FRAME_SIZE {
                            self.frame[i] = f[i];
                        }
                    }
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
        let mut frame = [0.0; audio_source::FRAME_SIZE];
        for i in 0..audio_source::FRAME_SIZE {
            frame[i] = self.frame[i];
        }
        self.audio_texture
            .upload_data(device, encoder, bytemuck::bytes_of(&frame));
    }
}
