use nannou::image;
use nannou::image::GenericImageView;
use nannou::prelude::*;
use tinyfiledialogs::open_file_dialog;

use crate::programs::config;
use crate::programs::uniforms::base::Bufferable;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub image1_size: Vector2,
    pub image2_size: Vector2,
}

pub struct ImageUniforms {
    pub data: Data,
    pub error: Option<nannou::image::ImageError>,
    pub image1_path: Option<String>,
    pub image1_texture: wgpu::Texture,
    pub image2_path: Option<String>,
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

// TODO show error in interface in non obtrusive way (bottom right)
impl ImageUniforms {
    pub fn new(device: &wgpu::Device) -> Self {
        let image1_texture = wgpu::TextureBuilder::new()
            .size([1, 1])
            .usage(wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED)
            .format(wgpu::TextureFormat::Rgba16Float)
            .build(device);

        let image2_texture = wgpu::TextureBuilder::new()
            .size([1, 1])
            .usage(wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED)
            .format(wgpu::TextureFormat::Rgba16Float)
            .build(device);

        Self {
            data: Data {
                image1_size: pt2(0.0, 0.0),
                image2_size: pt2(0.0, 0.0),
            },
            error: None,
            image1_path: None,
            image1_texture,
            image2_path: None,
            image2_texture,
            updated: false,
        }
    }

    pub fn load_image(&mut self, app: &App, image_id: i32, filepath: String) {
        let img = match image::open(&filepath) {
            Ok(img) => img,
            Err(e) => {
                self.error = Some(e);
                return;
            }
        };

        let (width, height) = img.dimensions();
        let texture = wgpu::Texture::from_image(app, &img);

        let filename = filepath.split('/').last().unwrap().to_string();
        let size = pt2(width as f32, height as f32);

        match image_id {
            1 => {
                self.image1_path = Some(filename);
                self.image1_texture = texture;
                self.data.image1_size = size;
            }
            2 => {
                self.image2_path = Some(filename);
                self.image2_texture = texture;
                self.data.image2_size = size;
            }
            _ => return,
        };

        self.updated = true;
    }

    pub fn set_defaults(&mut self, app: &App, defaults: &Option<config::ProgramDefaults>) {
        if let Some(cnfg) = defaults {
            let project_path = app.project_path().expect("failed to locate `project_path`");

            if let Some(img1) = &cnfg.image1 {
                self.load_image(
                    app,
                    1,
                    project_path
                        .join("images")
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
                        .join("images")
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
