use nannou::image;
use nannou::image::GenericImageView;
use nannou::prelude::*;
use nfd::Response;
use std::thread;

use crate::programs::config;
use crate::programs::uniforms::base::Bufferable;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub image1_size: Vector2,
}

pub struct ImageUniforms {
    pub data: Data,
    pub image1_path: Option<String>,
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
            image1_path: Some(String::from("fractal.png")),
            image1_texture,
            data: Data {
                image1_size: pt2(width as f32, height as f32),
            },
        }
    }

    pub fn load_image(&mut self) {
        // thread::spawn(move || {
        let result = nfd::open_file_dialog(None, None).unwrap_or_else(|e| {
            panic!(e);
        });

        match result {
            Response::Okay(file_path) => println!("File path = {:?}", file_path),
            Response::OkayMultiple(files) => println!("Files {:?}", files),
            Response::Cancel => println!("User canceled"),
        }
        // });
    }
}
