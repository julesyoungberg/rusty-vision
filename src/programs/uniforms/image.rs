use nannou::image;
use nannou::image::GenericImageView;
use nannou::prelude::*;
use tinyfiledialogs::open_file_dialog;

use crate::app;
use crate::programs::config;
use crate::programs::uniforms::base::Bufferable;
use crate::util;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub image1_size: Vector2<f32>,
    pub image2_size: Vector2<f32>,
}

pub struct ImageUniforms {
    pub data: Data,
    pub error: Option<String>,
    pub image1_name: Option<String>,
    pub image1_texture: wgpu::Texture,
    pub image2_name: Option<String>,
    pub image2_texture: wgpu::Texture,
    pub updated: bool,
}

impl Bufferable<Data> for ImageUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }

    fn textures(&self) -> Vec<&wgpu::Texture> {
        vec![&self.image1_texture, &self.image2_texture]
    }
}

impl ImageUniforms {
    pub fn new(device: &wgpu::Device) -> Self {
        let image1_texture = util::create_texture(device, [1, 1], wgpu::TextureFormat::Rgba16Float);
        let image2_texture = util::create_texture(device, [1, 1], wgpu::TextureFormat::Rgba16Float);

        Self {
            data: Data {
                image1_size: pt2(0.0, 0.0),
                image2_size: pt2(0.0, 0.0),
            },
            error: None,
            image1_name: None,
            image1_texture,
            image2_name: None,
            image2_texture,
            updated: false,
        }
    }

    pub fn load_image(&mut self, app: &App, image_id: i32, filepath: String) {
        let img = match image::open(&filepath) {
            Ok(img) => img,
            Err(e) => {
                self.error = Some(e.to_string());
                return;
            }
        };

        let (width, height) = img.dimensions();
        let texture = wgpu::Texture::from_image(app, &img);

        let filename = filepath.split('/').last().unwrap().to_string();
        let size = pt2(width as f32, height as f32);

        match image_id {
            1 => {
                self.image1_name = Some(filename);
                self.image1_texture = texture;
                self.data.image1_size = size;
            }
            2 => {
                self.image2_name = Some(filename);
                self.image2_texture = texture;
                self.data.image2_size = size;
            }
            _ => return,
        };

        self.updated = true;
    }

    pub fn configure(&mut self, app: &App, settings: &Option<config::ProgramSettings>) {
        if let Some(cnfg) = settings {
            let project_path = app.project_path().expect("failed to locate `project_path`");

            if let Some(img1) = &cnfg.image1 {
                self.load_image(
                    app,
                    1,
                    project_path
                        .join(app::MEDIA_DIR)
                        .join(img1)
                        .into_os_string()
                        .into_string()
                        .unwrap(),
                );
            }

            if let Some(img2) = &cnfg.image2 {
                self.load_image(
                    app,
                    2,
                    project_path
                        .join(app::MEDIA_DIR)
                        .join(img2)
                        .into_os_string()
                        .into_string()
                        .unwrap(),
                );
            }
        }
    }

    pub fn select_image(&mut self, app: &App, image_id: i32) {
        let filepath = match open_file_dialog("Load Image", "~", Some((&["*.jpg", "*.png"], ""))) {
            Some(filepath) => filepath,
            None => return,
        };

        println!("selected image: {:?}", filepath);

        self.load_image(app, image_id, filepath);
    }
}
