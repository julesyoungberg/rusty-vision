use nannou::math::cgmath::Matrix4;
use nannou::prelude::*;
use std::time::SystemTime;

use crate::config;
use crate::util;

/**
 * Generic interface
 */
pub trait Bufferable {
    fn as_bytes(&self) -> &[u8];

    fn set_program_defaults(&mut self, _selected: usize) {}
}

/**
 * General uniforms data
 */
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
    pub camera_pos_x: f32,
    pub camera_pos_y: f32,
    pub camera_pos_z: f32,
    pub camera_target_x: f32,
    pub camera_target_y: f32,
    pub camera_target_z: f32,
    pub camera_up_x: f32,
    pub camera_up_y: f32,
    pub camera_up_z: f32,
    pub rotation1_x: f32,
    pub rotation1_y: f32,
    pub rotation1_z: f32,
    pub rotation2_x: f32,
    pub rotation2_y: f32,
    pub rotation2_z: f32,
    pub offset1_x: f32,
    pub offset1_y: f32,
    pub offset1_z: f32,
    pub shape_rotation_x: f32,
    pub shape_rotation_y: f32,
    pub shape_rotation_z: f32,
}

/**
 * General Uniforms
 */
pub struct Uniforms {
    pub clock: SystemTime,
    pub data: Data,
}

impl Bufferable for Uniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }

    fn set_program_defaults(&mut self, selected: usize) {
        let defaults = config::PROGRAM_DEFAULTS[selected];

        self.data.camera_pos_x = defaults[0][0];
        self.data.camera_pos_y = defaults[0][1];
        self.data.camera_pos_z = defaults[0][2];

        self.data.camera_target_x = defaults[1][0];
        self.data.camera_target_y = defaults[1][1];
        self.data.camera_target_z = defaults[1][2];

        self.set_camera_up(pt3(defaults[2][0], defaults[2][1], defaults[2][2]));

        self.data.color_mode = defaults[4][0] as i32;
    }
}

impl Uniforms {
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
                camera_pos_x: 25.0,
                camera_pos_y: 0.0,
                camera_pos_z: 15.0,
                camera_target_x: 0.0,
                camera_target_y: 0.0,
                camera_target_z: 0.0,
                camera_up_x: 0.0,
                camera_up_y: 1.0,
                camera_up_z: 0.0,
                rotation1_x: 0.0,
                rotation1_y: 0.0,
                rotation1_z: 0.0,
                rotation2_x: 0.0,
                rotation2_y: 0.0,
                rotation2_z: 0.0,
                offset1_x: 0.0,
                offset1_y: 0.0,
                offset1_z: 0.0,
                shape_rotation_x: 0.0,
                shape_rotation_y: 0.0,
                shape_rotation_z: 0.0,
            },
        }
    }

    pub fn update_time(&mut self) {
        let elapsed = self.clock.elapsed().unwrap();
        self.data.time = elapsed.as_millis() as f32 / 1000.0;
    }

    pub fn camera_forward(&self) -> Vector3 {
        pt3(
            self.data.camera_target_x - self.data.camera_pos_x,
            self.data.camera_target_y - self.data.camera_pos_y,
            self.data.camera_target_z - self.data.camera_pos_z,
        )
    }

    pub fn camera_up(&self) -> Vector3 {
        pt3(
            self.data.camera_up_x,
            self.data.camera_up_y,
            self.data.camera_up_z,
        )
    }

    pub fn camera_dir(&self) -> Vector3 {
        util::normalize_vector(self.camera_forward())
    }

    pub fn set_camera_dir(&mut self, next_dir: Vector3) {
        let len = util::vector_length(self.camera_forward());
        let next_forward = pt3(next_dir.x * len, next_dir.y * len, next_dir.z * len);
        self.data.camera_target_x = self.data.camera_pos_x + next_forward.x;
        self.data.camera_target_y = self.data.camera_pos_y + next_forward.y;
        self.data.camera_target_z = self.data.camera_pos_z + next_forward.z;
    }

    pub fn set_camera_up(&mut self, next_dir: Vector3) {
        self.data.camera_up_x = next_dir.x;
        self.data.camera_up_y = next_dir.y;
        self.data.camera_up_z = next_dir.z;
    }

    pub fn translate_camera(&mut self, translation: Vector3) {
        self.data.camera_pos_x = self.data.camera_pos_x + translation.x;
        self.data.camera_pos_y = self.data.camera_pos_y + translation.y;
        self.data.camera_pos_z = self.data.camera_pos_z + translation.z;
        self.data.camera_target_x = self.data.camera_target_x + translation.x;
        self.data.camera_target_y = self.data.camera_target_y + translation.y;
        self.data.camera_target_z = self.data.camera_target_z + translation.z;
    }

    pub fn rotate_camera(&mut self, rotation: Matrix4<f32>) {
        self.set_camera_dir(util::transform_vector(&rotation, self.camera_dir()));
        self.set_camera_up(util::transform_vector(&rotation, self.camera_up()));
    }
}
