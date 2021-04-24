use nannou::prelude::*;
use ringbuf::{Consumer, RingBuffer};
use std::fmt;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use crate::programs::uniforms::audio_source;
use crate::programs::uniforms::base::Bufferable;
use crate::util;

pub struct AudioUniforms {
    pub audio_texture: wgpu::Texture,

    audio_channel_tx: Option<Sender<audio_source::AudioMessage>>,
    audio_consumer: Option<Consumer<Vec<f32>>>,
    audio_thread: Option<std::thread::JoinHandle<()>>,
    frame: Vec<f32>,
}

impl fmt::Debug for AudioUniforms {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AudioUniforms")
    }
}

impl Bufferable for AudioUniforms {
    fn textures(&self) -> Vec<&wgpu::Texture> {
        vec![&self.audio_texture]
    }
}

impl AudioUniforms {
    pub fn new(device: &wgpu::Device) -> Self {
        let audio_texture = util::create_texture(
            device,
            [audio_source::FRAME_SIZE as u32, 1],
            wgpu::TextureFormat::R32Float,
        );

        Self {
            audio_channel_tx: None,
            audio_consumer: None,
            audio_texture,
            audio_thread: None,
            frame: vec![0.0; audio_source::FRAME_SIZE],
        }
    }

    pub fn start_session(&mut self, audio_source: &mut audio_source::AudioSource) {
        let (audio_channel_tx, audio_channel_rx) = channel();
        audio_source.subscribe(String::from("audio"), audio_channel_tx.clone());
        self.audio_channel_tx = Some(audio_channel_tx);

        let ring_buffer = RingBuffer::<Vec<f32>>::new(2);
        let (mut producer, consumer) = ring_buffer.split();
        producer.push(vec![0.0; audio_source::FRAME_SIZE]).unwrap();
        self.audio_consumer = Some(consumer);

        self.audio_thread = Some(thread::spawn(move || {
            for msg in audio_channel_rx.iter() {
                match msg {
                    audio_source::AudioMessage::Data(frame) => {
                        producer.push(frame).ok();
                    }
                    audio_source::AudioMessage::Close | audio_source::AudioMessage::Error(_) => {
                        break;
                    }
                }
            }
        }));
    }

    pub fn end_session(&mut self, audio_source: &mut audio_source::AudioSource) {
        audio_source.unsubscribe(String::from("audio"));

        if let Some(channel) = &self.audio_channel_tx {
            channel.send(audio_source::AudioMessage::Close).unwrap();
        }

        if let Some(handle) = self.audio_thread.take() {
            handle.join().unwrap();
        }
    }

    pub fn update(&mut self) {
        if let Some(mut c) = self.audio_consumer.take() {
            let popped = c.pop();
            self.audio_consumer = Some(c);

            if let Some(f) = popped {
                if f.len() > 0 {
                    self.frame[..audio_source::FRAME_SIZE]
                        .clone_from_slice(&f[..audio_source::FRAME_SIZE]);
                }
            }
        };
    }

    pub fn update_texture(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        let mut frame = [0.0; audio_source::FRAME_SIZE];
        frame[..audio_source::FRAME_SIZE].clone_from_slice(&self.frame[..audio_source::FRAME_SIZE]);
        self.audio_texture
            .upload_data(device, encoder, bytemuck::bytes_of(&frame));
    }
}
