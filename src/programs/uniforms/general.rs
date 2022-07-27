use nannou::prelude::*;
use std::time::SystemTime;

use crate::programs::uniforms::base::Bufferable;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub mouse: Vector2<f32>,
    pub resolution: Vector2<f32>,
    pub time: f32,
    pub mouse_down: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct GeneralUniforms {
    pub clock: SystemTime,
    pub data: Data,

    paused_at: f32,
    paused_time: f32,
    reset_at: f32,
}

impl Bufferable<Data> for GeneralUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }
}

impl GeneralUniforms {
    pub fn new(resolution: Vector2<f32>) -> Self {
        println!("resolution: {:?}", resolution);
        Self {
            clock: SystemTime::now(),
            data: Data {
                mouse: pt2(0.0, 0.0),
                resolution,
                time: 0.0,
                mouse_down: 0,
            },
            paused_at: 0.0,
            paused_time: 0.0,
            reset_at: 0.0,
        }
    }

    fn get_time(&self) -> f32 {
        let elapsed = self.clock.elapsed().unwrap();
        elapsed.as_secs_f32() - self.reset_at - self.paused_time
    }

    pub fn update(&mut self) {
        self.data.time = self.get_time();
    }

    pub fn set_size(&mut self, size: Vector2<f32>) {
        self.data.resolution = size;
    }

    pub fn set_mouse(&mut self, mouse: Vector2<f32>) {
        self.data.mouse = mouse;
    }

    pub fn reset(&mut self) {
        self.reset_at = self.data.time;
    }

    pub fn pause(&mut self) {
        self.paused_at = self.data.time;
    }

    pub fn unpause(&mut self) {
        self.paused_time += self.get_time() - self.paused_at;
    }
}
