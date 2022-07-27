use nannou::math::cgmath::Matrix4;
use nannou::prelude::*;

use crate::programs::config;
use crate::programs::uniforms::base::Bufferable;
use crate::util;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
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

#[derive(Debug, Copy, Clone)]
pub struct CameraUniforms {
    pub data: Data,
}

impl Bufferable<Data> for CameraUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }
}

impl CameraUniforms {
    pub fn new() -> Self {
        Self {
            data: Data {
                camera_pos_x: 25.0,
                camera_pos_y: 0.0,
                camera_pos_z: 15.0,
                camera_target_x: 0.0,
                camera_target_y: 0.0,
                camera_target_z: 0.0,
                camera_up_x: 0.0,
                camera_up_y: 1.0,
                camera_up_z: 0.0,
            },
        }
    }

    pub fn configure(&mut self, settings: &Option<config::ProgramSettings>) {
        let mut camera_pos = pt3(0.0, 0.0, 5.0);
        let mut camera_target = pt3(0.0, 0.0, 0.0);
        let mut camera_up = pt3(0.0, 1.0, 0.0);

        if let Some(cnfg) = settings {
            if let Some(cam_pos) = cnfg.camera_position {
                camera_pos = cam_pos;
            }

            if let Some(cam_target) = cnfg.camera_target {
                camera_target = cam_target;
            }

            if let Some(cam_up) = cnfg.camera_up {
                camera_up = cam_up;
            }
        }

        self.data.camera_pos_x = camera_pos[0];
        self.data.camera_pos_y = camera_pos[1];
        self.data.camera_pos_z = camera_pos[2];

        self.data.camera_target_x = camera_target[0];
        self.data.camera_target_y = camera_target[1];
        self.data.camera_target_z = camera_target[2];

        self.set_up(camera_up);
    }

    pub fn forward(&self) -> Vector3<f32> {
        pt3(
            self.data.camera_target_x - self.data.camera_pos_x,
            self.data.camera_target_y - self.data.camera_pos_y,
            self.data.camera_target_z - self.data.camera_pos_z,
        )
    }

    pub fn up(&self) -> Vector3<f32> {
        pt3(
            self.data.camera_up_x,
            self.data.camera_up_y,
            self.data.camera_up_z,
        )
    }

    pub fn dir(&self) -> Vector3<f32> {
        util::normalize_vector(self.forward())
    }

    pub fn set_dir(&mut self, next_dir: Vector3<f32>) {
        let len = util::vector_length(self.forward());
        let next_forward = pt3(next_dir.x * len, next_dir.y * len, next_dir.z * len);
        self.data.camera_target_x += next_forward.x;
        self.data.camera_target_y += next_forward.y;
        self.data.camera_target_z += next_forward.z;
    }

    pub fn set_up(&mut self, next_dir: Vector3<f32>) {
        self.data.camera_up_x = next_dir.x;
        self.data.camera_up_y = next_dir.y;
        self.data.camera_up_z = next_dir.z;
    }

    pub fn translate(&mut self, translation: Vector3<f32>) {
        self.data.camera_pos_x += translation.x;
        self.data.camera_pos_y += translation.y;
        self.data.camera_pos_z += translation.z;
        self.data.camera_target_x += translation.x;
        self.data.camera_target_y += translation.y;
        self.data.camera_target_z += translation.z;
    }

    pub fn rotate(&mut self, rotation: Matrix4<f32>) {
        self.set_dir(util::transform_vector(&rotation, self.dir()));
        self.set_up(util::transform_vector(&rotation, self.up()));
    }
}
