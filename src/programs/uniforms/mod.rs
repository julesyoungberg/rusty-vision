use nannou::prelude::*;

use crate::programs::config;

pub mod audio;
pub mod audio_features;
pub mod audio_fft;
mod audio_source;
pub mod base;
pub mod camera;
pub mod color;
pub mod general;
pub mod geometry;
pub mod image;
pub mod multipass;
pub mod noise;
pub mod video;
pub mod video_capture;
pub mod webcam;

use base::{UniformBuffers, UniformBuffersMethods};

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
    pub video: bool,
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
        video: false,
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
        "video" => subscriptions.video = true,
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
    pub video_uniforms: video::VideoUniforms,
    pub webcam_uniforms: webcam::WebcamUniforms,
}

impl BufferStore {
    pub fn new(device: &wgpu::Device, size: Vector2) -> Self {
        let audio_source = audio_source::AudioSource::new();

        // store buffers in map
        let mut buffers = UniformBuffers::new();

        // create uniforms and buffers
        let audio_uniforms = audio::AudioUniforms::new(device);
        buffers.add(device, "audio", &audio_uniforms);

        let audio_features_uniforms = audio_features::AudioFeaturesUniforms::new(device);
        buffers.add(device, "audio_features", &audio_features_uniforms);

        let audio_fft_uniforms = audio_fft::AudioFftUniforms::new(device);
        buffers.add(device, "audio_fft", &audio_fft_uniforms);

        let camera_uniforms = camera::CameraUniforms::new();
        buffers.add(device, "camera", &camera_uniforms);

        let color_uniforms = color::ColorUniforms::new();
        buffers.add(device, "color", &color_uniforms);

        let general_uniforms = general::GeneralUniforms::new(size);
        buffers.add(device, "general", &general_uniforms);

        let geometry_uniforms = geometry::GeometryUniforms::new();
        buffers.add(device, "geometry", &geometry_uniforms);

        let image_uniforms = image::ImageUniforms::new(device);
        buffers.add(device, "image", &image_uniforms);

        let multipass_uniforms = multipass::MultipassUniforms::new(size);
        buffers.add(device, "multipass", &multipass_uniforms);

        let noise_uniforms = noise::NoiseUniforms::new();
        buffers.add(device, "noise", &noise_uniforms);

        let video_uniforms = video::VideoUniforms::new();
        buffers.add(device, "video", &video_uniforms);

        let webcam_uniforms = webcam::WebcamUniforms::new();
        buffers.add(device, "webcam", &webcam_uniforms);

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
            video_uniforms,
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
    pub fn configure(
        &mut self,
        app: &App,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        subscriptions: &UniformSubscriptions,
        settings: &Option<config::ProgramSettings>,
        size: Point2,
        num_samples: u32,
    ) {
        self.end_audio_session();
        self.audio_features_uniforms.configure(settings);
        self.audio_fft_uniforms.configure(settings);
        self.start_audio_session(subscriptions);

        self.camera_uniforms.configure(settings);

        self.color_uniforms.configure(settings);

        self.image_uniforms.configure(app, settings);

        self.multipass_uniforms
            .configure(settings, device, encoder, size, num_samples);
        self.buffers
            .add(device, "multipass", &self.multipass_uniforms);

        self.noise_uniforms.configure(settings);

        self.video_uniforms.end_session();
        if subscriptions.video {
            self.video_uniforms.configure(app, device, settings);
            if self.video_uniforms.updated {
                self.buffers.add(device, "video", &self.video_uniforms);
            }
        }

        if subscriptions.webcam {
            self.webcam_uniforms.configure(device, size);
            if self.webcam_uniforms.updated {
                self.buffers.add(device, "webcam", &self.webcam_uniforms);
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

        if subscriptions.image && self.image_uniforms.updated {
            self.buffers.add(device, "image", &self.image_uniforms);
        }

        if subscriptions.multipass {
            self.multipass_uniforms
                .update(device, encoder, size, num_samples);
            if self.multipass_uniforms.updated {
                self.buffers
                    .add(device, "multipass", &self.multipass_uniforms);
            }
        }

        if subscriptions.video {
            self.video_uniforms.update();
            if self.video_uniforms.updated {
                self.buffers.add(device, "video", &self.video_uniforms);
            }
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
        encoder: &mut wgpu::CommandEncoder,
        subscriptions: &UniformSubscriptions,
    ) {
        if subscriptions.audio {
            self.audio_uniforms.update_texture(device, encoder);
        }

        if subscriptions.audio_features {
            self.audio_features_uniforms.update_texture(device, encoder);
            self.buffers.update(
                device,
                encoder,
                "audio_features",
                &self.audio_features_uniforms,
            );
        }

        if subscriptions.audio_fft {
            self.audio_fft_uniforms.update_texture(device, encoder);
        }

        if subscriptions.camera {
            self.buffers
                .update(device, encoder, "camera", &self.camera_uniforms);
        }

        if subscriptions.color {
            self.buffers
                .update(device, encoder, "color", &self.color_uniforms);
        }

        if subscriptions.general {
            self.buffers
                .update(device, encoder, "general", &self.general_uniforms);
        }

        if subscriptions.geometry {
            self.buffers
                .update(device, encoder, "geometry", &self.geometry_uniforms);
        }

        if subscriptions.multipass {
            self.buffers
                .update(device, encoder, "multipass", &self.multipass_uniforms);
        }

        if subscriptions.noise {
            self.buffers
                .update(device, encoder, "noise", &self.noise_uniforms);
        }

        if subscriptions.video {
            self.video_uniforms.update_texture(device, encoder);
            self.buffers
                .update(device, encoder, "video", &self.video_uniforms);
        }

        if subscriptions.webcam {
            self.webcam_uniforms.update_texture(device, encoder);
            self.buffers
                .update(device, encoder, "webcam", &self.webcam_uniforms);
        }
    }

    pub fn pause(&mut self, subscriptions: &UniformSubscriptions) {
        if subscriptions.audio || subscriptions.audio_features || subscriptions.audio_fft {
            self.end_audio_session();
        }

        self.general_uniforms.pause();

        if subscriptions.video {
            self.video_uniforms.pause();
        }

        if subscriptions.webcam {
            self.webcam_uniforms.pause();
        }
    }

    pub fn unpause(&mut self, subscriptions: &UniformSubscriptions) {
        if subscriptions.audio || subscriptions.audio_features || subscriptions.audio_fft {
            self.start_audio_session(subscriptions);
        }

        self.general_uniforms.unpause();

        if subscriptions.video {
            self.video_uniforms.unpause();
        }

        if subscriptions.webcam {
            self.webcam_uniforms.unpause();
        }
    }

    pub fn updated(&self) -> bool {
        self.image_uniforms.updated
            || self.multipass_uniforms.updated
            || self.video_uniforms.updated
            || self.webcam_uniforms.updated
    }

    pub fn finish_update(&mut self) {
        self.image_uniforms.updated = false;
        self.multipass_uniforms.updated = false;
        self.video_uniforms.updated = false;
        self.webcam_uniforms.updated = false;
    }

    pub fn end_session(&mut self) {
        self.end_audio_session();
        self.video_uniforms.end_session();
        self.webcam_uniforms.end_session();
    }
}
