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
pub mod image;
pub mod noise;
pub mod webcam;

/// Stores a uniform buffer along with the relevant bind groups.
/// Maintains the uniform buffer.
/// Since this application requires many uniform buffers but nearly no
/// other type of GPU data, it makes sense to wrap the bind_group and
/// layout all into one object with generic functionality.
#[derive(Debug)]
pub struct UniformBuffer {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub buffer: wgpu::Buffer,
}

impl UniformBuffer {
    pub fn new<T>(device: &wgpu::Device, uniforms: &impl Bufferable<T>) -> Self
    where
        T: Copy,
    {
        let data = uniforms.as_bytes();
        let textures = uniforms.textures();
        let usage = wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST;
        let buffer = device.create_buffer_with_data(data, usage);

        let mut layout_builder = wgpu::BindGroupLayoutBuilder::new();
        let mut texture_views = vec![];

        if textures.len() > 0 {
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

        let bind_group_layout = layout_builder
            .uniform_buffer(wgpu::ShaderStage::FRAGMENT, false)
            .build(device);

        let mut group_builder = wgpu::BindGroupBuilder::new();
        let sampler = wgpu::SamplerBuilder::new().build(device);

        if texture_views.len() > 0 {
            group_builder = group_builder.sampler(&sampler);
            for texture_view in texture_views.iter() {
                group_builder = group_builder.texture_view(texture_view);
            }
        }

        let bind_group = group_builder
            .buffer::<T>(&buffer, 0..1)
            .build(device, &bind_group_layout);

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
        encoder: &mut nannou::wgpu::CommandEncoder,
        uniforms: &impl Bufferable<T>,
    ) {
        let size = std::mem::size_of::<T>() as wgpu::BufferAddress;
        let usage = wgpu::BufferUsage::COPY_SRC;
        let next_buffer = device.create_buffer_with_data(uniforms.as_bytes(), usage);
        encoder.copy_buffer_to_buffer(&next_buffer, 0, &self.buffer, 0, size);
    }
}

pub type UniformBuffers = HashMap<String, UniformBuffer>;

/// Defines a program's subscriptions to uniform data.
/// This determines which data should be fetched / updated.
#[derive(Debug)]
pub struct UniformSubscriptions {
    pub audio: bool,
    pub camera: bool,
    pub color: bool,
    pub general: bool,
    pub geometry: bool,
    pub image: bool,
    pub noise: bool,
    pub webcam: bool,
}

/// Build a subscriptions struct from a list of uniform names
pub fn get_subscriptions(names: &Vec<String>) -> UniformSubscriptions {
    let mut subscriptions = UniformSubscriptions {
        audio: false,
        camera: false,
        color: false,
        geometry: false,
        general: false,
        image: false,
        noise: false,
        webcam: false,
    };

    names.iter().for_each(|n| match n.as_str() {
        "audio" => subscriptions.audio = true,
        "camera" => subscriptions.camera = true,
        "color" => subscriptions.color = true,
        "general" => subscriptions.general = true,
        "geometry" => subscriptions.geometry = true,
        "image" => subscriptions.image = true,
        "noise" => subscriptions.noise = true,
        "webcam" => subscriptions.webcam = true,
        _ => (),
    });

    subscriptions
}

/// Stores all different uniforms.
/// Mantains the uniform data and the corresponding GPU buffers.
pub struct BufferStore {
    pub audio_uniforms: audio::AudioUniforms,
    pub buffers: UniformBuffers,
    pub camera_uniforms: camera::CameraUniforms,
    pub color_uniforms: color::ColorUniforms,
    pub general_uniforms: general::GeneralUniforms,
    pub geometry_uniforms: geometry::GeometryUniforms,
    pub image_uniforms: image::ImageUniforms,
    pub noise_uniforms: noise::NoiseUniforms,
    pub webcam_uniforms: webcam::WebcamUniforms,
}

impl BufferStore {
    pub fn new(device: &wgpu::Device) -> Self {
        // create uniforms and buffers
        let audio_uniforms = audio::AudioUniforms::new(device);
        let audio_uniform_buffer = UniformBuffer::new(device, &audio_uniforms);

        let camera_uniforms = camera::CameraUniforms::new();
        let camera_uniform_buffer = UniformBuffer::new(device, &camera_uniforms);

        let color_uniforms = color::ColorUniforms::new();
        let color_uniform_buffer = UniformBuffer::new(device, &color_uniforms);

        let general_uniforms = general::GeneralUniforms::new(pt2(
            app_config::SIZE[0] as f32,
            app_config::SIZE[1] as f32,
        ));
        let general_uniform_buffer = UniformBuffer::new(device, &general_uniforms);

        let geometry_uniforms = geometry::GeometryUniforms::new();
        let geometry_uniform_buffer = UniformBuffer::new(device, &geometry_uniforms);

        let image_uniforms = image::ImageUniforms::new(device);
        let image_uniform_buffer = UniformBuffer::new(device, &image_uniforms);

        let noise_uniforms = noise::NoiseUniforms::new();
        let noise_uniform_buffer = UniformBuffer::new(device, &noise_uniforms);

        let webcam_uniforms = webcam::WebcamUniforms::new();
        let webcam_uniform_buffer = UniformBuffer::new(device, &webcam_uniforms);

        // store buffers in map
        let mut buffers = HashMap::new();
        buffers.insert(String::from("audio"), audio_uniform_buffer);
        buffers.insert(String::from("camera"), camera_uniform_buffer);
        buffers.insert(String::from("color"), color_uniform_buffer);
        buffers.insert(String::from("general"), general_uniform_buffer);
        buffers.insert(String::from("geometry"), geometry_uniform_buffer);
        buffers.insert(String::from("image"), image_uniform_buffer);
        buffers.insert(String::from("noise"), noise_uniform_buffer);
        buffers.insert(String::from("webcam"), webcam_uniform_buffer);

        Self {
            audio_uniforms,
            buffers,
            camera_uniforms,
            color_uniforms,
            general_uniforms,
            geometry_uniforms,
            image_uniforms,
            noise_uniforms,
            webcam_uniforms,
        }
    }

    /// Set default uniforms for current selected program.
    /// Also a place to do any initialization and/or cleanup.
    pub fn set_program_defaults(
        &mut self,
        app: &App,
        device: &wgpu::Device,
        subscriptions: &UniformSubscriptions,
        defaults: &Option<config::ProgramDefaults>,
    ) {
        if subscriptions.audio {
            self.audio_uniforms.set_defaults(defaults);
        } else {
            self.audio_uniforms.end_session();
        }

        if subscriptions.camera {
            self.camera_uniforms.set_defaults(defaults);
        }

        if subscriptions.color {
            self.color_uniforms.set_defaults(defaults);
        }

        if subscriptions.image {
            self.image_uniforms.set_defaults(app, defaults);
        }

        if subscriptions.webcam {
            self.webcam_uniforms.set_defaults(device, defaults);
            if self.webcam_uniforms.updated {
                let webcam_uniform_buffer = UniformBuffer::new(device, &self.webcam_uniforms);
                self.buffers
                    .insert(String::from("webcam"), webcam_uniform_buffer);
            }
        }
    }

    /// Update uniform data.
    /// Call every timestep.
    pub fn update(&mut self, device: &wgpu::Device, subscriptions: &UniformSubscriptions) {
        if subscriptions.audio {
            self.audio_uniforms.update();
        }

        if subscriptions.general {
            self.general_uniforms.update();
        }

        if subscriptions.image && self.image_uniforms.updated {
            // recreate the uniform buffer object
            let image_uniform_buffer = UniformBuffer::new(device, &self.image_uniforms);
            self.buffers
                .insert(String::from("image"), image_uniform_buffer);
        }

        if subscriptions.webcam {
            self.webcam_uniforms.update();
        }
    }

    /// Update GPU uniform buffers with current data.
    /// Call in draw() before rendering.
    pub fn update_buffers(
        &self,
        device: &wgpu::Device,
        encoder: &mut nannou::wgpu::CommandEncoder,
        subscriptions: &UniformSubscriptions,
    ) {
        if subscriptions.audio {
            self.audio_uniforms.update_textures(device, encoder);
            self.buffers
                .get("audio")
                .unwrap()
                .update(device, encoder, &self.audio_uniforms);
        }

        if subscriptions.camera {
            self.buffers
                .get("camera")
                .unwrap()
                .update(device, encoder, &self.camera_uniforms);
        }

        if subscriptions.color {
            self.buffers
                .get("color")
                .unwrap()
                .update(device, encoder, &self.color_uniforms);
        }

        if subscriptions.general {
            self.buffers
                .get("general")
                .unwrap()
                .update(device, encoder, &self.general_uniforms);
        }

        if subscriptions.geometry {
            self.buffers
                .get("geometry")
                .unwrap()
                .update(device, encoder, &self.geometry_uniforms);
        }

        if subscriptions.noise {
            self.buffers
                .get("noise")
                .unwrap()
                .update(device, encoder, &self.noise_uniforms);
        }

        if subscriptions.webcam {
            self.webcam_uniforms.update_texture(device, encoder);
            self.buffers
                .get("webcam")
                .unwrap()
                .update(device, encoder, &self.webcam_uniforms);
        }
    }
}
