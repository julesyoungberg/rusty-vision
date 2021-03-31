use nannou::prelude::*;
use std::collections::HashMap;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {}

/// Generic interface
pub trait Bufferable<T = Data>: Sized {
    fn as_bytes(&self) -> &[u8] {
        &[]
    }

    fn textures(&self) -> Vec<&wgpu::Texture> {
        vec![]
    }
}

/// Stores a uniform buffer along with the relevant bind groups.
/// Maintains the uniform buffer.
/// Since this application requires many uniform buffers but nearly no
/// other type of GPU data, it makes sense to wrap the bind_group and
/// layout all into one object with generic functionality.
#[derive(Debug)]
pub struct UniformBuffer {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub buffer: Option<wgpu::Buffer>,
}

impl UniformBuffer {
    pub fn new<T>(device: &wgpu::Device, uniforms: &impl Bufferable<T>) -> Self
    where
        T: Copy,
    {
        let data = uniforms.as_bytes();
        let textures = uniforms.textures();

        let mut layout_builder = wgpu::BindGroupLayoutBuilder::new();
        let mut texture_views = vec![];

        if !textures.is_empty() {
            layout_builder = layout_builder.sampler(wgpu::ShaderStage::FRAGMENT);

            for texture in textures.iter() {
                let texture_view = texture.view().build();
                layout_builder = layout_builder.sampled_texture(
                    wgpu::ShaderStage::FRAGMENT,
                    false,
                    wgpu::TextureViewDimension::D2,
                    texture_view.component_type(),
                );
                texture_views.push(texture_view);
            }
        }

        let mut buffer = None;
        if !data.is_empty() {
            layout_builder = layout_builder.uniform_buffer(wgpu::ShaderStage::FRAGMENT, false);
            let usage = wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST;
            let buff = device.create_buffer_with_data(data, usage);
            buffer = Some(buff);
        }

        let bind_group_layout = layout_builder.build(device);

        let mut group_builder = wgpu::BindGroupBuilder::new();
        let sampler = wgpu::SamplerBuilder::new().build(device);

        if !texture_views.is_empty() {
            group_builder = group_builder.sampler(&sampler);
            for texture_view in texture_views.iter() {
                group_builder = group_builder.texture_view(texture_view);
            }
        }

        if let Some(buff) = &buffer {
            group_builder = group_builder.buffer::<T>(buff, 0..1);
        }

        let bind_group = group_builder.build(device, &bind_group_layout);

        Self {
            bind_group,
            bind_group_layout,
            buffer,
        }
    }

    /// updates the buffer with new data
    pub fn update<T>(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        uniforms: &impl Bufferable<T>,
    ) {
        if let Some(buffer) = &self.buffer {
            let size = std::mem::size_of::<T>() as wgpu::BufferAddress;
            let usage = wgpu::BufferUsage::COPY_SRC;
            let next_buffer = device.create_buffer_with_data(uniforms.as_bytes(), usage);
            encoder.copy_buffer_to_buffer(&next_buffer, 0, buffer, 0, size);
        }
    }
}

pub type UniformBuffers = HashMap<String, UniformBuffer>;

pub trait UniformBuffersMethods {
    fn add<T>(&mut self, device: &wgpu::Device, name: &str, uniforms: &impl Bufferable<T>)
    where
        T: Copy;

    fn update<T>(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        name: &str,
        uniforms: &impl Bufferable<T>,
    ) where
        T: Copy;
}

impl UniformBuffersMethods for UniformBuffers {
    fn add<T>(&mut self, device: &wgpu::Device, name: &str, uniforms: &impl Bufferable<T>)
    where
        T: Copy,
    {
        let uniform_buffer = UniformBuffer::new(device, uniforms);
        self.insert(String::from(name), uniform_buffer);
    }

    fn update<T>(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        name: &str,
        uniforms: &impl Bufferable<T>,
    ) where
        T: Copy,
    {
        self.get(name).unwrap().update(device, encoder, uniforms);
    }
}
