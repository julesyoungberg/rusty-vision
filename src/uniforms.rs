#![allow(dead_code)]
use nannou::prelude::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Data {
    pub time: f32,
}

pub struct Uniforms {
    pub data: Data,
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            data: Data { time: 0.0 },
        }
    }

    pub fn update_time(&mut self) {
        self.data.time = 0.5;
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }
}
