use nannou::prelude::*;

use crate::config;
use crate::programs::uniforms::base::Bufferable;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub color_mode: i32,
    pub color1_r: f32,
    pub color1_g: f32,
    pub color1_b: f32,
    pub color2_r: f32,
    pub color2_g: f32,
    pub color2_b: f32,
    pub color3_r: f32,
    pub color3_g: f32,
    pub color3_b: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct ColorUniforms {
    pub data: Data,
}

impl Bufferable for ColorUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }

    fn set_program_defaults(&mut self, selected: usize) {
        let defaults = config::PROGRAM_DEFAULTS[selected];

        self.data.color_mode = defaults[4][0] as i32;
    }
}

impl ColorUniforms {
    pub fn new() -> Self {
        Self {
            data: Data {
                color_mode: 0,
                color1_r: 1.0,
                color1_g: 0.0,
                color1_b: 0.0,
                color2_r: 0.0,
                color2_g: 1.0,
                color2_b: 0.0,
                color3_r: 0.0,
                color3_g: 0.0,
                color3_b: 1.0,
            },
        }
    }
}
