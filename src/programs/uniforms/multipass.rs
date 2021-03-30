use nannou::prelude::*;

use crate::programs::config;
use crate::programs::uniforms::base::Bufferable;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub pass_index: i32,
}

#[derive(Debug, Clone)]
pub struct MultipassUniforms {
    pub data: Data,
    pub passes: i32,

    size: Point2,
    textures: Vec<wgpu::Texture>,
}

impl Bufferable<Data> for MultipassUniforms {
    // fn as_bytes(&self) -> &[u8] {
    //     unsafe { wgpu::bytes::from(&self.data) }
    // }

    fn textures(&self) -> Vec<&wgpu::Texture> {
        self.textures.iter().collect::<Vec<&wgpu::Texture>>()
    }
}

impl MultipassUniforms {
    pub fn new(size: Point2) -> Self {
        Self {
            data: Data { pass_index: 0 },
            passes: 0,
            size,
            textures: vec![],
        }
    }

    fn configure(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        size: Point2,
        num_samples: u32,
    ) {
        self.size = size;
        self.data.pass_index = 0;
        self.textures = vec![];

        for _ in 0..self.passes {
            let texture = wgpu::TextureBuilder::new()
                .size([size[0] as u32, size[1] as u32])
                .sample_count(num_samples)
                .format(Frame::TEXTURE_FORMAT)
                .usage(wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED)
                .build(device);
            let data = vec![0u8; texture.size_bytes()];
            texture.upload_data(device, encoder, &data);
            self.textures.push(texture);
        }
    }

    pub fn set_defaults(
        &mut self,
        defaults: &Option<config::ProgramDefaults>,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        size: Point2,
        num_samples: u32,
    ) {
        self.passes = match defaults {
            Some(cnfg) => match cnfg.passes {
                Some(p) => p,
                None => return,
            },
            None => return,
        };

        self.configure(device, encoder, size, num_samples);
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        size: Point2,
        num_samples: u32,
    ) {
        if self.size != size {
            self.configure(device, encoder, size, num_samples);
        }
    }
}
