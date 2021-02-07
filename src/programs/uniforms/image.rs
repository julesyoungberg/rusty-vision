use nannou::image;
use nannou::image::GenericImageView;
use nannou::prelude::*;

use crate::programs::config;
use crate::programs::uniforms::base::Bufferable;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub image1_size: Vector2,
}

pub struct ImageUniforms {
    pub data: Data,
    pub image1_texture: wgpu::Texture,
}

impl Bufferable for ImageUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }

    fn set_program_defaults(&mut self, _defaults: &Option<config::ProgramDefaults>) {}
}

impl ImageUniforms {
    pub fn new(app: &App) -> Self {
        let image = image::open("./images/fractal.png").unwrap();
        let (width, height) = image.dimensions();

        let image1_texture = wgpu::Texture::from_image(app, &image);

        Self {
            image1_texture,
            data: Data {
                image1_size: pt2(width as f32, height as f32),
            },
        }
    }
}
