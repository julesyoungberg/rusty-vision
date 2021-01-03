#![allow(dead_code)]
use nannou::prelude::*;
use std::time::SystemTime;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub color_mode: i32,
    pub draw_floor: bool,
    pub fog_dist: f32,
    pub quality: f32,
    pub resolution: Vector2,
    pub time: f32,
    pub color1_r: f32,
    pub color1_g: f32,
    pub color1_b: f32,
    pub color2_r: f32,
    pub color2_g: f32,
    pub color2_b: f32,
    pub color3_r: f32,
    pub color3_g: f32,
    pub color3_b: f32,
    pub camera_pos_x: f32,
    pub camera_pos_y: f32,
    pub camera_pos_z: f32,
    pub camera_target_x: f32,
    pub camera_target_y: f32,
    pub camera_target_z: f32,
    pub camera_up_x: f32,
    pub camera_up_y: f32,
    pub camera_up_z: f32,
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
                fog_dist: 150.0,
                quality: 1.0,
                resolution,
                time: 0.0,
                color1_r: 1.0,
                color1_g: 0.0,
                color1_b: 0.0,
                color2_r: 0.0,
                color2_g: 1.0,
                color2_b: 0.0,
                color3_r: 0.0,
                color3_g: 0.0,
                color3_b: 1.0,
                camera_pos_x: 20.0,
                camera_pos_y: 0.0,
                camera_pos_z: 10.0,
                camera_target_x: 0.0,
                camera_target_y: 0.0,
                camera_target_z: 0.0,
                camera_up_x: 0.0,
                camera_up_y: 1.0,
                camera_up_z: 0.0,
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

    pub fn camera_dir(&self) -> Vector3 {
        let dir = pt3(
            self.data.camera_target_x - self.data.camera_pos_x,
            self.data.camera_target_y - self.data.camera_pos_y,
            self.data.camera_target_z - self.data.camera_pos_z,
        );
        let sum = dir.x * dir.x + dir.y * dir.y + dir.z * dir.z;
        let len = sum.sqrt();
        pt3(dir.x / len, dir.y / len, dir.z / len)
    }
}
