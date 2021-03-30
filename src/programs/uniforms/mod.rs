use nannou::prelude::*;
use std::collections::HashMap;

use crate::programs::config;
use crate::programs::uniforms::base::Bufferable;

pub mod audio;
pub mod audio_features;
pub mod audio_fft;
pub mod audio_source;
pub mod base;
pub mod camera;
pub mod color;
pub mod general;
pub mod geometry;
pub mod image;
pub mod multipass;
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
        encoder: &mut nannou::wgpu::CommandEncoder,
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

/// Defines a program's subscriptions to uniform data.
/// This determines which data should be fetched / updated.
#[derive(Debug)]
pub struct UniformSubscriptions {
    pub audio: bool,
    pub audio_features: bool,
    pub audio_fft: bool,
    pub camera: bool,
    pub color: bool,
    pub general: bool,
    pub geometry: bool,
    pub image: bool,
    pub noise: bool,
    pub multipass: bool,
    pub webcam: bool,
}

/// Build a subscriptions struct from a list of uniform names
pub fn get_subscriptions(names: &[String]) -> UniformSubscriptions {
    let mut subscriptions = UniformSubscriptions {
        audio: false,
        audio_features: false,
        audio_fft: false,
        camera: false,
        color: false,
        geometry: false,
        general: false,
        image: false,
        noise: false,
        multipass: false,
        webcam: false,
    };

    names.iter().for_each(|n| match n.as_str() {
        "audio" => subscriptions.audio = true,
        "audio_features" => subscriptions.audio_features = true,
        "audio_fft" => subscriptions.audio_fft = true,
        "camera" => subscriptions.camera = true,
        "color" => subscriptions.color = true,
        "general" => subscriptions.general = true,
        "geometry" => subscriptions.geometry = true,
        "image" => subscriptions.image = true,
        "noise" => subscriptions.noise = true,
        "multipass" => subscriptions.multipass = true,
        "webcam" => subscriptions.webcam = true,
        _ => (),
    });

    subscriptions
}

/// Stores all different uniforms.
/// Mantains the uniform data and the corresponding GPU buffers.
pub struct BufferStore {
    pub audio_features_uniforms: audio_features::AudioFeaturesUniforms,
    pub audio_fft_uniforms: audio_fft::AudioFftUniforms,
    pub audio_source: audio_source::AudioSource,
    pub audio_uniforms: audio::AudioUniforms,
    pub buffers: UniformBuffers,
    pub camera_uniforms: camera::CameraUniforms,
    pub color_uniforms: color::ColorUniforms,
    pub general_uniforms: general::GeneralUniforms,
    pub geometry_uniforms: geometry::GeometryUniforms,
    pub image_uniforms: image::ImageUniforms,
    pub noise_uniforms: noise::NoiseUniforms,
    pub multipass_uniforms: multipass::MultipassUniforms,
    pub webcam_uniforms: webcam::WebcamUniforms,
}

impl BufferStore {
    pub fn new(device: &wgpu::Device, size: Vector2) -> Self {
        let audio_source = audio_source::AudioSource::new();

        // create uniforms and buffers
        let audio_uniforms = audio::AudioUniforms::new(device);
        let audio_uniform_buffer = UniformBuffer::new(device, &audio_uniforms);

        let audio_features_uniforms = audio_features::AudioFeaturesUniforms::new(device);
        let audio_features_uniform_buffer = UniformBuffer::new(device, &audio_features_uniforms);

        let audio_fft_uniforms = audio_fft::AudioFftUniforms::new(device);
        let audio_fft_uniform_buffer = UniformBuffer::new(device, &audio_fft_uniforms);

        let camera_uniforms = camera::CameraUniforms::new();
        let camera_uniform_buffer = UniformBuffer::new(device, &camera_uniforms);

        let color_uniforms = color::ColorUniforms::new();
        let color_uniform_buffer = UniformBuffer::new(device, &color_uniforms);

        let general_uniforms = general::GeneralUniforms::new(size);
        let general_uniform_buffer = UniformBuffer::new(device, &general_uniforms);

        let geometry_uniforms = geometry::GeometryUniforms::new();
        let geometry_uniform_buffer = UniformBuffer::new(device, &geometry_uniforms);

        let image_uniforms = image::ImageUniforms::new(device);
        let image_uniform_buffer = UniformBuffer::new(device, &image_uniforms);

        let multipass_uniforms = multipass::MultipassUniforms::new(size);
        let multipass_uniform_buffer = UniformBuffer::new(device, &multipass_uniforms);

        let noise_uniforms = noise::NoiseUniforms::new();
        let noise_uniform_buffer = UniformBuffer::new(device, &noise_uniforms);

        let webcam_uniforms = webcam::WebcamUniforms::new();
        let webcam_uniform_buffer = UniformBuffer::new(device, &webcam_uniforms);

        // store buffers in map
        let mut buffers = HashMap::new();
        buffers.insert(String::from("audio"), audio_uniform_buffer);
        buffers.insert(
            String::from("audio_features"),
            audio_features_uniform_buffer,
        );
        buffers.insert(String::from("audio_fft"), audio_fft_uniform_buffer);
        buffers.insert(String::from("camera"), camera_uniform_buffer);
        buffers.insert(String::from("color"), color_uniform_buffer);
        buffers.insert(String::from("general"), general_uniform_buffer);
        buffers.insert(String::from("geometry"), geometry_uniform_buffer);
        buffers.insert(String::from("image"), image_uniform_buffer);
        buffers.insert(String::from("noise"), noise_uniform_buffer);
        buffers.insert(String::from("multipass"), multipass_uniform_buffer);
        buffers.insert(String::from("webcam"), webcam_uniform_buffer);

        Self {
            audio_uniforms,
            audio_features_uniforms,
            audio_fft_uniforms,
            audio_source,
            buffers,
            camera_uniforms,
            color_uniforms,
            general_uniforms,
            geometry_uniforms,
            image_uniforms,
            multipass_uniforms,
            noise_uniforms,
            webcam_uniforms,
        }
    }

    pub fn start_audio_session(&mut self, subscriptions: &UniformSubscriptions) {
        let mut audio_channels = vec![];
        let mut error_channels = vec![];

        if subscriptions.audio {
            audio_channels.push(self.audio_uniforms.start_session());
        }

        if subscriptions.audio_features {
            let (audio_channel, error_channel) = self.audio_features_uniforms.create_channels();
            audio_channels.push(audio_channel);
            error_channels.push(error_channel);
        }

        if subscriptions.audio_fft {
            audio_channels.push(self.audio_fft_uniforms.start_session());
        }

        if !audio_channels.is_empty()
            && !self
                .audio_source
                .start_session(audio_channels, error_channels)
        {
            self.end_audio_session();
        }

        if subscriptions.audio_features
            && !self
                .audio_features_uniforms
                .start_session(self.audio_source.sample_rate)
        {
            self.end_audio_session();
        }
    }

    pub fn end_audio_session(&mut self) {
        self.audio_source.end_session();
        self.audio_uniforms.end_session();
        self.audio_features_uniforms.end_session();
        self.audio_fft_uniforms.end_session();
    }

    /// Set default uniforms for current selected program.
    /// Also a place to do any initialization and/or cleanup.
    pub fn set_program_defaults(
        &mut self,
        app: &App,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        subscriptions: &UniformSubscriptions,
        defaults: &Option<config::ProgramDefaults>,
        size: Point2,
        num_samples: u32,
    ) {
        self.end_audio_session();
        self.audio_features_uniforms.set_defaults(defaults);
        self.audio_fft_uniforms.set_defaults(defaults);
        self.start_audio_session(subscriptions);

        self.camera_uniforms.set_defaults(defaults);

        self.color_uniforms.set_defaults(defaults);

        self.image_uniforms.set_defaults(app, defaults);

        self.multipass_uniforms
            .set_defaults(defaults, device, encoder, size, num_samples);

        self.noise_uniforms.set_defaults(defaults);

        if subscriptions.webcam {
            self.webcam_uniforms.set_defaults(device, defaults);
            if self.webcam_uniforms.updated {
                let webcam_uniform_buffer = UniformBuffer::new(device, &self.webcam_uniforms);
                self.buffers
                    .insert(String::from("webcam"), webcam_uniform_buffer);
            }
        } else {
            self.webcam_uniforms.end_session();
        }
    }

    /// Update uniform data.
    /// Call every timestep.
    pub fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        subscriptions: &UniformSubscriptions,
        size: Point2,
        num_samples: u32,
    ) {
        if subscriptions.audio || subscriptions.audio_features || subscriptions.audio_fft {
            self.audio_source.update();
        }

        if subscriptions.audio {
            self.audio_uniforms.update();
        }

        if subscriptions.audio_features {
            self.audio_features_uniforms.update();
        }

        if subscriptions.audio_fft {
            self.audio_fft_uniforms.update();
        }

        if subscriptions.general {
            self.general_uniforms.update();
        }

        if subscriptions.multipass {
            self.multipass_uniforms
                .update(device, encoder, size, num_samples);
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
            self.audio_uniforms.update_texture(device, encoder);
        }

        if subscriptions.audio_features {
            self.audio_features_uniforms.update_texture(device, encoder);
            self.buffers.get("audio_features").unwrap().update(
                device,
                encoder,
                &self.audio_features_uniforms,
            );
        }

        if subscriptions.audio_fft {
            self.audio_fft_uniforms.update_texture(device, encoder);
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

        if subscriptions.multipass {
            self.buffers.get("multipass").unwrap().update(
                device,
                encoder,
                &self.multipass_uniforms,
            );
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

    pub fn pause(&mut self, subscriptions: &UniformSubscriptions) {
        if subscriptions.audio || subscriptions.audio_features || subscriptions.audio_fft {
            self.end_audio_session();
        }

        self.general_uniforms.pause();

        if subscriptions.webcam {
            self.webcam_uniforms.pause();
        }
    }

    pub fn unpause(&mut self, subscriptions: &UniformSubscriptions) {
        if subscriptions.audio || subscriptions.audio_features || subscriptions.audio_fft {
            self.start_audio_session(subscriptions);
        }

        self.general_uniforms.unpause();

        if subscriptions.webcam {
            self.webcam_uniforms.unpause();
        }
    }
}
