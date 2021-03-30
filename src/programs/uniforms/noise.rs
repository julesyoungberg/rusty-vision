use nannou::prelude::*;

use crate::programs::config;
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

impl Bufferable<Data> for NoiseUniforms {
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
                scale_by_prev: 0,
                sharpen: 0,
                speed: 0.1,
            },
        }
    }

    pub fn configure(&mut self, settings: &Option<config::ProgramSettings>) {
        if let Some(cnfg) = settings {
            if let Some(lacunarity) = cnfg.noise_lacunarity {
                self.data.lacunarity = lacunarity;
            }

            if let Some(gain) = cnfg.noise_gain {
                self.data.gain = gain;
            }

            if let Some(invert) = cnfg.noise_invert {
                self.data.invert = invert;
            }

            if let Some(mirror) = cnfg.noise_mirror {
                self.data.mirror = mirror;
            }

            if let Some(octaves) = cnfg.noise_octaves {
                self.data.octaves = octaves;
            }

            if let Some(scale_by_prev) = cnfg.noise_scale_by_prev {
                self.data.scale_by_prev = scale_by_prev;
            }

            if let Some(sharpen) = cnfg.noise_sharpen {
                self.data.sharpen = sharpen;
            }

            if let Some(speed) = cnfg.noise_speed {
                self.data.speed = speed;
            }
        }
    }
}
