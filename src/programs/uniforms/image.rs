use nannou::image;
use nannou::image::GenericImageView;
use nannou::prelude::*;
use tinyfiledialogs;

use crate::programs::uniforms::base::Bufferable;
use crate::util;

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

    fn textures(&self) -> Option<Vec<&wgpu::Texture>> {
        Some(vec![&self.image1_texture, &self.image2_texture])
    }
}

impl ImageUniforms {
    pub fn new(app: &App) -> Self {
        let img1 =
            image::open(util::universal_path(String::from("./images/fractal1.png"))).unwrap();
        let (width1, height1) = img1.dimensions();
        let image1_texture = wgpu::Texture::from_image(app, &img1);

        let img2 =
            image::open(util::universal_path(String::from("./images/fractal2.png"))).unwrap();
        let (width2, height2) = img2.dimensions();
        let image2_texture = wgpu::Texture::from_image(app, &img2);

        Self {
            data: Data {
                image1_size: pt2(width1 as f32, height1 as f32),
                image2_size: pt2(width2 as f32, height2 as f32),
            },
            error: None,
            image1_path: Some(String::from("fractal1.png")),
            image1_texture,
            image2_path: Some(String::from("fractal2.png")),
            image2_texture,
            updated: false,
        }
    }

    pub fn load_image(&mut self, app: &App, image_id: i32) {
        let filepath = match tinyfiledialogs::open_file_dialog(
            "Load Image",
            "~",
            Some((&["*.jpg", "*.png"], "")),
        ) {
            Some(filepath) => filepath,
            None => return,
        };

        println!("selected image: {:?}", filepath);

        let img = match image::open(&filepath) {
            Ok(img) => img,
            Err(e) => {
                self.error = Some(e);
                return;
            }
        };

        let (width, height) = img.dimensions();
        let texture = wgpu::Texture::from_image(app, &img);

        let filename = filepath.split("/").last().unwrap().to_string();
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

        println!("updated image texture");
        self.updated = true;
    }
}
