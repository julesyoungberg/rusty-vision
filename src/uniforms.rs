#![allow(dead_code)]
use nannou::prelude::*;

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    pub time: f32,
}

impl Uniforms {
    pub fn new() -> Self {
        Self { time: 0.0 }
    }

    pub fn update_time(&mut self) {
        self.time = self.time + 0.1;
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(self) }
    }
}
