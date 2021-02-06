use nannou::prelude::*;

use crate::programs::uniforms::base::Bufferable;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub lacunarity: f32,
    pub gain: f32,
    pub invert: i32,
    pub mirror: i32,
    pub octaves: i32,
    pub scale_by_prev: i32,
    pub sharpen: i32,
    pub speed: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct NoiseUniforms {
    pub data: Data,
}

impl Bufferable for NoiseUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }
}

impl NoiseUniforms {
    pub fn new() -> Self {
        Self {
            data: Data {
                lacunarity: 2.0,
                gain: 0.5,
                invert: 0,
                mirror: 0,
                octaves: 4,
                scale_by_prev: 1,
                sharpen: 1,
                speed: 0.1,
            },
        }
    }
}
