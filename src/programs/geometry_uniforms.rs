use nannou::prelude::*;

use crate::programs::uniforms;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub rotation1_x: f32,
    pub rotation1_y: f32,
    pub rotation1_z: f32,
    pub rotation2_x: f32,
    pub rotation2_y: f32,
    pub rotation2_z: f32,
    pub offset1_x: f32,
    pub offset1_y: f32,
    pub offset1_z: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct GeometryUniforms {
    pub data: Data,
}

impl uniforms::Bufferable for GeometryUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }
}

impl GeometryUniforms {
    pub fn new() -> Self {
        Self {
            data: Data {
                rotation1_x: 0.0,
                rotation1_y: 0.0,
                rotation1_z: 0.0,
                rotation2_x: 0.0,
                rotation2_y: 0.0,
                rotation2_z: 0.0,
                offset1_x: 0.0,
                offset1_y: 0.0,
                offset1_z: 0.0,
            },
        }
    }
}
