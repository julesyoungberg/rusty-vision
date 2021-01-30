use nannou::prelude::*;
use std::collections::HashMap;

use crate::app_config;
use crate::programs::config;
use crate::programs::uniforms::base::Bufferable;

pub mod audio;
pub mod base;
pub mod camera;
pub mod color;
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
 * Defines a program's subscriptions to uniform data.
 * This determines which data should be fetched / updated.
 */
#[derive(Debug)]
pub struct UniformSubscriptions {
    pub audio: bool,
    pub camera: bool,
    pub color: bool,
    pub general: bool,
    pub geometry: bool,
}

/**
 * Build a subscriptions struct from a list of uniform names
 */
pub fn get_subscriptions(names: &Vec<String>) -> UniformSubscriptions {
    let mut subscriptions = UniformSubscriptions {
        audio: false,
        camera: false,
        color: false,
        geometry: false,
        general: false,
    };

    if names.iter().any(|n| n == "audio") {
        subscriptions.audio = true;
    }

    if names.iter().any(|n| n == "camera") {
        subscriptions.camera = true;
    }

    if names.iter().any(|n| n == "color") {
        subscriptions.color = true;
    }

    if names.iter().any(|n| n == "general") {
        subscriptions.general = true;
    }

    if names.iter().any(|n| n == "geometry") {
        subscriptions.geometry = true;
    }

    subscriptions
}

/**
 * Stores all different uniforms
 */
pub struct BufferStore {
    pub audio_uniforms: audio::AudioUniforms,
    pub buffers: UniformBuffers,
    pub camera_uniforms: camera::CameraUniforms,
    pub color_uniforms: color::ColorUniforms,
    pub general_uniforms: general::GeneralUniforms,
    pub geometry_uniforms: geometry::GeometryUniforms,
}

/**
 * Mantains the uniform data and the corresponding GPU buffers
 */
impl BufferStore {
    pub fn new(device: &wgpu::Device) -> Self {
        // create uniforms and buffers
        let audio_uniforms = audio::AudioUniforms::new();
        let audio_uniform_buffer =
            UniformBuffer::new::<audio::Data>(device, audio_uniforms.as_bytes());

        let camera_uniforms = camera::CameraUniforms::new();
        let camera_uniform_buffer =
            UniformBuffer::new::<camera::Data>(device, camera_uniforms.as_bytes());

        let color_uniforms = color::ColorUniforms::new();
        let color_uniform_buffer =
            UniformBuffer::new::<color::Data>(device, color_uniforms.as_bytes());

        let general_uniforms = general::GeneralUniforms::new(pt2(
            app_config::SIZE[0] as f32,
            app_config::SIZE[1] as f32,
        ));
        let general_uniform_buffer =
            UniformBuffer::new::<general::Data>(device, general_uniforms.as_bytes());

        let geometry_uniforms = geometry::GeometryUniforms::new();
        let geometry_uniform_buffer =
            UniformBuffer::new::<geometry::Data>(device, geometry_uniforms.as_bytes());

        // store buffers in map
        let mut buffers = HashMap::new();
        buffers.insert(String::from("audio"), audio_uniform_buffer);
        buffers.insert(String::from("camera"), camera_uniform_buffer);
        buffers.insert(String::from("color"), color_uniform_buffer);
        buffers.insert(String::from("general"), general_uniform_buffer);
        buffers.insert(String::from("geometry"), geometry_uniform_buffer);

        Self {
            audio_uniforms,
            buffers,
            camera_uniforms,
            color_uniforms,
            general_uniforms,
            geometry_uniforms,
        }
    }

    /**
     * Set default uniforms for current selected program.
     * Also a place to do any initialization and/or cleanup.
     */
    pub fn set_program_defaults(
        &mut self,
        subscriptions: &UniformSubscriptions,
        defaults: &Option<config::ProgramDefaults>,
    ) {
        if subscriptions.camera {
            self.camera_uniforms.set_program_defaults(defaults);
        }

        if subscriptions.color {
            self.color_uniforms.set_program_defaults(defaults);
        }

        if subscriptions.general {
            self.general_uniforms.set_program_defaults(defaults);
        }

        if subscriptions.audio {
            self.audio_uniforms.set_program_defaults(defaults);
        } else {
            self.audio_uniforms.end_session();
        }
    }

    /**
     * Update uniform data.
     * Call every timestep.
     */
    pub fn update(&mut self, subscriptions: &UniformSubscriptions) {
        if subscriptions.general {
            self.general_uniforms.update();
        }

        if subscriptions.audio {
            self.audio_uniforms.update();
        }
    }

    /**
     * Update GPU uniform buffers with current data.
     * Call in draw() before rendering.
     */
    pub fn update_buffers(
        &self,
        device: &wgpu::Device,
        encoder: &mut nannou::wgpu::CommandEncoder,
        subscriptions: &UniformSubscriptions,
    ) {
        if subscriptions.camera {
            self.buffers.get("camera").unwrap().update::<camera::Data>(
                device,
                encoder,
                self.camera_uniforms.as_bytes(),
            );
        }

        if subscriptions.color {
            self.buffers.get("color").unwrap().update::<color::Data>(
                device,
                encoder,
                self.color_uniforms.as_bytes(),
            );
        }

        if subscriptions.general {
            self.buffers
                .get("general")
                .unwrap()
                .update::<general::Data>(device, encoder, self.general_uniforms.as_bytes());
        }

        if subscriptions.geometry {
            self.buffers
                .get("geometry")
                .unwrap()
                .update::<geometry::Data>(device, encoder, self.geometry_uniforms.as_bytes());
        }

        if subscriptions.audio {
            self.buffers.get("audio").unwrap().update::<audio::Data>(
                device,
                encoder,
                self.audio_uniforms.as_bytes(),
            );
        }
    }
}
