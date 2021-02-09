use nannou::image;
use nannou::image::GenericImageView;
use nannou::prelude::*;
use tinyfiledialogs;

use crate::programs::config;
use crate::programs::uniforms::base::Bufferable;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub image1_size: Vector2,
}

pub struct ImageUniforms {
    pub data: Data,
    pub error: Option<nannou::image::ImageError>,
    pub image1_path: Option<String>,
    pub image1_texture: wgpu::Texture,
    pub updated: bool,
}

impl Bufferable for ImageUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }

    fn set_program_defaults(&mut self, _defaults: &Option<config::ProgramDefaults>) {}
}

impl ImageUniforms {
    pub fn new(app: &App) -> Self {
        let img = image::open("./images/fractal.png").unwrap();
        let (width, height) = img.dimensions();

        let image1_texture = wgpu::Texture::from_image(app, &img);

        Self {
            data: Data {
                image1_size: pt2(width as f32, height as f32),
            },
            error: None,
            image1_path: Some(String::from("fractal.png")),
            image1_texture,
            updated: false,
        }
    }

    pub fn load_image(&mut self, app: &App, image_id: i32) {
        let filename = match tinyfiledialogs::open_file_dialog(
            "Load Image",
            "~",
            Some((&["*.jpg", "*.png"], "")),
        ) {
            Some(filename) => filename,
            None => return,
        };

        println!("selected image: {:?}", filename);

        let img = match image::open(&filename) {
            Ok(img) => img,
            Err(e) => {
                self.error = Some(e);
                return;
            }
        };

        let (width, height) = img.dimensions();
        let texture = wgpu::Texture::from_image(app, &img);

        match image_id {
            1 => {
                self.image1_path = Some(filename);
                self.image1_texture = texture;
                self.data.image1_size = pt2(width as f32, height as f32);
            }
            _ => return,
        };

        println!("updated image texture");
        self.updated = true;
    }
}
