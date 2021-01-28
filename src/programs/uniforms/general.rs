use nannou::prelude::*;
use std::time::SystemTime;

use crate::config;
use crate::programs::uniforms::base::Bufferable;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub color_mode: i32,
    pub draw_floor: i32,
    pub fog_dist: f32,
    pub time: f32,
    pub resolution: Vector2,
    pub color1_r: f32,
    pub color1_g: f32,
    pub color1_b: f32,
    pub color2_r: f32,
    pub color2_g: f32,
    pub color2_b: f32,
    pub color3_r: f32,
    pub color3_g: f32,
    pub color3_b: f32,
    pub shape_rotation_x: f32,
    pub shape_rotation_y: f32,
    pub shape_rotation_z: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct GeneralUniforms {
    pub clock: SystemTime,
    pub data: Data,
}

impl Bufferable for GeneralUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }

    fn set_program_defaults(&mut self, selected: usize) {
        let defaults = config::PROGRAM_DEFAULTS[selected];

        self.data.color_mode = defaults[4][0] as i32;
    }
}

impl GeneralUniforms {
    pub fn new(resolution: Vector2) -> Self {
        Self {
            clock: SystemTime::now(),
            data: Data {
                color_mode: 0,
                draw_floor: 1,
                fog_dist: 150.0,
                time: 0.0,
                resolution,
                color1_r: 1.0,
                color1_g: 0.0,
                color1_b: 0.0,
                color2_r: 0.0,
                color2_g: 1.0,
                color2_b: 0.0,
                color3_r: 0.0,
                color3_g: 0.0,
                color3_b: 1.0,
                shape_rotation_x: 0.0,
                shape_rotation_y: 0.0,
                shape_rotation_z: 0.0,
            },
        }
    }

    pub fn update(&mut self) {
        let elapsed = self.clock.elapsed().unwrap();
        self.data.time = elapsed.as_millis() as f32 / 1000.0;
    }
}
