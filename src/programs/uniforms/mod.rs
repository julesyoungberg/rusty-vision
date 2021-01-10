use nannou::prelude::*;
use std::collections::HashMap;

use crate::config;
use crate::programs::uniforms::base::Bufferable;

pub mod base;
pub mod camera;
pub mod general;
pub mod geometry;

/**
 * Stores a uniform buffer along with the relevant bind groups.
 */
#[derive(Debug)]
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
#[derive(Debug)]
pub struct BufferStore {
    pub buffers: UniformBuffers,
    pub camera_uniforms: camera::Uniforms,
    pub general_uniforms: general::Uniforms,
    pub geometry_uniforms: geometry::Uniforms,
}

/**
 * Mantains the uniform data and the corresponding GPU buffers
 */
impl BufferStore {
    pub fn new(device: &wgpu::Device) -> Self {
        let mut camera_uniforms = camera::Uniforms::new();
        camera_uniforms.set_program_defaults(config::DEFAULT_PROGRAM);
        let camera_uniform_buffer =
            UniformBuffer::new::<camera::Data>(device, camera_uniforms.as_bytes());

        let mut general_uniforms =
            general::Uniforms::new(pt2(config::SIZE[0] as f32, config::SIZE[1] as f32));
        general_uniforms.set_program_defaults(config::DEFAULT_PROGRAM);
        let general_uniform_buffer =
            UniformBuffer::new::<general::Data>(device, general_uniforms.as_bytes());

        let mut geometry_uniforms = geometry::Uniforms::new();
        geometry_uniforms.set_program_defaults(config::DEFAULT_PROGRAM);
        let geometry_uniform_buffer =
            UniformBuffer::new::<geometry::Data>(device, geometry_uniforms.as_bytes());

        let mut buffers = HashMap::new();
        buffers.insert(String::from("camera"), camera_uniform_buffer);
        buffers.insert(String::from("general"), general_uniform_buffer);
        buffers.insert(String::from("geometry"), geometry_uniform_buffer);

        Self {
            buffers,
            camera_uniforms,
            general_uniforms,
            geometry_uniforms,
        }
    }

    /**
     * Set default uniforms for current selected program
     * TODO: only modify the uniforms used by the current program
     */
    pub fn set_program_defaults(&mut self, selected: usize) {
        self.camera_uniforms.set_program_defaults(selected);
        self.general_uniforms.set_program_defaults(selected);
    }

    /**
     * Update uniform data.
     * Call every timestep.
     * TODO: only update uniforms by current program
     */
    pub fn update(&mut self) {
        self.general_uniforms.update();
    }

    /**
     * Update GPU uniform buffers with current data.
     * Call in draw() before rendering.
     * TODO: only update buffers used by current program
     */
    pub fn update_buffers(
        &self,
        device: &wgpu::Device,
        encoder: &mut nannou::wgpu::CommandEncoder,
    ) {
        self.buffers.get("camera").unwrap().update::<camera::Data>(
            device,
            encoder,
            self.camera_uniforms.as_bytes(),
        );

        self.buffers
            .get("general")
            .unwrap()
            .update::<general::Data>(device, encoder, self.general_uniforms.as_bytes());

        self.buffers
            .get("geometry")
            .unwrap()
            .update::<geometry::Data>(device, encoder, self.geometry_uniforms.as_bytes());
    }
}
