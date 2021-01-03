#![allow(dead_code)]
use nannou::prelude::*;
use std::time::SystemTime;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub color_mode: u32,
    pub draw_floor: bool,
    pub fog_dist: f32,
    pub quality: u32,
    pub resolution: Vector2,
    pub spin: bool,
    pub time: f32,
}

pub struct Uniforms {
    pub clock: SystemTime,
    pub data: Data,
}

impl Uniforms {
    pub fn new(resolution: Vector2) -> Self {
        Self {
            clock: SystemTime::now(),
            data: Data {
                color_mode: 0,
                draw_floor: true,
                fog_dist: 50.0,
                quality: 1,
                resolution,
                spin: false,
                time: 0.0,
            },
        }
    }

    pub fn update_time(&mut self) {
        let elapsed = self.clock.elapsed().unwrap();
        self.data.time = elapsed.as_millis() as f32 / 1000.0;
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }
}
