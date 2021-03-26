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
    pub passes: u32,

    textures: Vec<wgpu::Texture>,
}

impl Bufferable<Data> for MultipassUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }

    fn textures(&self) -> Vec<&wgpu::Texture> {
        self.textures.iter().collect::<Vec<&wgpu::Texture>>()
    }
}

impl MultipassUniforms {
    pub fn new() -> Self {
        Self {
            data: Data { pass_index: 0 },
            passes: 0,
            textures: vec![],
        }
    }

    pub fn set_defaults(
        &mut self,
        defaults: &Option<config::ProgramDefaults>,
        device: &wgpu::Device,
        size: Point2,
    ) {
        self.data.pass_index = 0;
        self.passes = 0;
        self.textures = vec![];

        self.passes = match defaults {
            Some(cnfg) => match cnfg.passes {
                Some(p) => p,
                None => return,
            },
            None => return,
        };

        for _ in 0..self.passes {
            let texture = wgpu::TextureBuilder::new()
                .size([size[0] as u32, size[1] as u32])
                .format(wgpu::TextureFormat::Rgba32Float)
                .usage(
                    wgpu::TextureUsage::COPY_DST
                        | wgpu::TextureUsage::SAMPLED
                        | wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                )
                .build(device);
            self.textures.push(texture);
        }
    }
}
