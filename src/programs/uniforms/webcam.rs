use nannou::prelude::*;
use opencv;

use crate::programs::uniforms::base::Bufferable;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub image_size: Vector2,
}

pub struct WebcamUniforms {
    capture: opencv::videoio::VideoCapture,
    data: Data,
}

impl Bufferable<Data> for WebcamUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }

    fn textures(&self) -> Option<Vec<&wgpu::Texture>> {
        None
    }
}

impl WebcamUniforms {
    pub fn new(device: &wgpu::Device) -> Self {
        let capture = opencv::videoio::VideoCapture::default().unwrap();

        Self {
            capture,
            data: Data {
                image_size: pt2(0.0, 0.0),
            },
        }
    }
}
