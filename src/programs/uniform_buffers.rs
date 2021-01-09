use nannou::prelude::*;
use std::collections::HashMap;

use crate::config;
use crate::programs::geometry_uniforms;
use crate::programs::uniforms;
use crate::programs::uniforms::Bufferable;

/**
 * Stores a uniform buffer along with the relevant bind groups.
 */
pub struct UniformBuffer {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub buffer: wgpu::Buffer,
}

/**
 * Maintains the uniform buffer.
 * Since this application requires many uniform buffers but nearly no
 * other type of GPU data, it makes sense to wrap the bind_group and
 * layout all into one object with generic functionality.
 */
impl UniformBuffer {
    pub fn new<T>(device: &wgpu::Device, data: &[u8]) -> Self
    where
        T: Copy,
    {
        let usage = wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST;
        let buffer = device.create_buffer_with_data(data, usage);

        let bind_group_layout = wgpu::BindGroupLayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStage::FRAGMENT, false)
            .build(device);

        let bind_group = wgpu::BindGroupBuilder::new()
            .buffer::<T>(&buffer, 0..1)
            .build(device, &bind_group_layout);

        Self {
            bind_group,
            bind_group_layout,
            buffer,
        }
    }

    /**
     * updates the buffer with new data
     */
    pub fn update<T>(
        &self,
        device: &wgpu::Device,
        encoder: &mut nannou::wgpu::CommandEncoder,
        data: &[u8],
    ) {
        let size = std::mem::size_of::<T>() as wgpu::BufferAddress;
        let usage = wgpu::BufferUsage::COPY_SRC;
        let next_buffer = device.create_buffer_with_data(data, usage);
        encoder.copy_buffer_to_buffer(&next_buffer, 0, &self.buffer, 0, size);
    }
}

pub type UniformBuffers = HashMap<String, UniformBuffer>;

/**
 * Stores all different uniforms
 */
pub struct UniformBufferStore {
    pub buffers: UniformBuffers,
    pub geometry_uniforms: geometry_uniforms::GeometryUniforms,
    pub uniforms: uniforms::Uniforms,
}

/**
 * Mantains the uniform data and the corresponding GPU buffers
 */
impl UniformBufferStore {
    pub fn new(device: &wgpu::Device) -> Self {
        let mut uniforms =
            uniforms::Uniforms::new(pt2(config::SIZE[0] as f32, config::SIZE[1] as f32));
        uniforms.set_program_defaults(config::DEFAULT_PROGRAM);
        let uniform_buffer = UniformBuffer::new::<uniforms::Data>(device, uniforms.as_bytes());

        let mut geometry_uniforms = geometry_uniforms::GeometryUniforms::new();
        geometry_uniforms.set_program_defaults(config::DEFAULT_PROGRAM);
        let geometry_uniform_buffer =
            UniformBuffer::new::<geometry_uniforms::Data>(device, geometry_uniforms.as_bytes());

        let mut buffers = HashMap::new();
        buffers.insert(String::from("general"), uniform_buffer);
        buffers.insert(String::from("geometry"), geometry_uniform_buffer);

        Self {
            buffers,
            geometry_uniforms,
            uniforms,
        }
    }

    pub fn set_program_defaults(&mut self, selected: usize) {
        self.uniforms.set_program_defaults(selected);
    }

    /**
     * Update uniform data.
     * Call every timestep.
     */
    pub fn update(&mut self) {
        self.uniforms.update();
    }

    /**
     * Update GPU uniform buffers with current data.
     * TODO: figure out a way to do this iteratively so that it can be left untouched when adding new buffers
     * Call in draw() before rendering.
     */
    pub fn update_buffers(
        &self,
        device: &wgpu::Device,
        encoder: &mut nannou::wgpu::CommandEncoder,
    ) {
        self.buffers
            .get("general")
            .unwrap()
            .update::<uniforms::Data>(device, encoder, self.uniforms.as_bytes());

        self.buffers
            .get("geometry")
            .unwrap()
            .update::<geometry_uniforms::Data>(device, encoder, self.geometry_uniforms.as_bytes());
    }
}
