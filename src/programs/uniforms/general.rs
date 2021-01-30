use nannou::prelude::*;
use std::time::SystemTime;

use crate::programs::config;
use crate::programs::uniforms::base::Bufferable;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub resolution: Vector2,
    pub time: f32,
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

    fn set_program_defaults(&mut self, _defaults: &Option<config::ProgramDefaults>) {}
}

impl GeneralUniforms {
    pub fn new(resolution: Vector2) -> Self {
        Self {
            clock: SystemTime::now(),
            data: Data {
                resolution,
                time: 0.0,
            },
        }
    }

    pub fn update(&mut self) {
        let elapsed = self.clock.elapsed().unwrap();
        self.data.time = elapsed.as_millis() as f32 / 1000.0;
    }
}
