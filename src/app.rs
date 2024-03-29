use nannou::prelude::*;
use nannou::ui::prelude::*;
use std::cell::Ref;

use crate::interface;
use crate::programs;
use crate::quad_2d;
use crate::util;

pub const MEDIA_DIR: &str = "media";

widget_ids! {
    /// UI widget ids
    pub struct WidgetIds {
        color_mode,
        color_mode_label,
        current_program,
        current_program_label,
        current_folder,
        current_folder_label,
        draw_floor,
        draw_floor_label,
        fog_dist,
        audio_fft_folder,
        general_folder,
        geometry_folder,
        image_folder,
        noise_folder,
        video_folder,
        color1_r,
        color1_g,
        color1_b,
        color2_r,
        color2_g,
        color2_b,
        color3_r,
        color3_g,
        color3_b,
        color1_label,
        color2_label,
        color3_label,
        toggle_controls_hint,
        controls_container,
        controls_wrapper,
        rotation1_label,
        rotation1_x,
        rotation1_y,
        rotation1_z,
        rotation2_label,
        rotation2_x,
        rotation2_y,
        rotation2_z,
        offset1_label,
        offset1_x,
        offset1_y,
        offset1_z,
        info_wrapper,
        camera_pos_display,
        camera_target_display,
        camera_up_display,
        shape_rotation_label,
        shape_rotation_x,
        shape_rotation_y,
        shape_rotation_z,
        errors_wrapper,
        errors_title,
        errors_message,
        audio_feature_smoothing,
        audio_fft_smoothing,
        noise_lacunarity,
        noise_gain,
        noise_invert,
        noise_invert_label,
        noise_mirror,
        noise_mirror_label,
        noise_octaves,
        noise_scale_by_prev,
        noise_scale_by_prev_label,
        noise_sharpen,
        noise_sharpen_label,
        noise_speed,
        image1_label,
        image1_load_button,
        image2_label,
        image2_load_button,
        video_label,
        video_load_button,
        video_speed,
        fps_container,
        fps,
        isf_inputs_title,
    }
}

/// Main application state
pub struct Model {
    pub widget_ids: WidgetIds,
    pub main_window_id: WindowId,
    pub original_height: u32,
    pub original_width: u32,
    pub paused: bool,
    pub program_store: programs::ProgramStore,
    pub show_controls: bool,
    pub texture: wgpu::Texture,
    pub texture_reshaper: wgpu::TextureReshaper,
    pub ui: Ui,
    pub ui_show_audio_fft: bool,
    pub ui_show_color: bool,
    pub ui_show_geometry: bool,
    pub ui_show_image: bool,
    pub ui_show_noise: bool,
    pub ui_show_video: bool,
    pub resized: bool,
    pub size: Vector2<f32>,
    pub vertex_buffer: wgpu::Buffer,
}

impl Model {
    /// Update app state.
    pub fn encode_update(
        &mut self,
        app: &App,
        update: Update,
        window: &Ref<'_, Window>,
        device: &wgpu::Device,
        num_samples: u32,
    ) {
        self.program_store.reset_data();

        let desc = wgpu::CommandEncoderDescriptor {
            label: Some("rusty_vision_update"),
        };
        let mut encoder = device.create_command_encoder(&desc);

        if self.show_controls {
            interface::update(app, device, &mut encoder, self, num_samples);
        }

        self.program_store
            .encode_update(app, update, device, &mut encoder, self.size, num_samples);

        if self.resized {
            let msaa_samples = window.msaa_samples();
            self.texture = util::create_app_texture(device, self.size, msaa_samples);
            self.texture_reshaper =
                util::create_texture_reshaper(device, &self.texture, msaa_samples);
            self.resized = false;
        }

        window.swap_chain_queue().submit(vec![encoder.finish()]);
    }

    /// Encode a render pass to a given texture.
    pub fn encode_render_pass(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        texture_view: &wgpu::TextureView,
    ) {
        if let Some(isf_pipeline) = &self.program_store.isf_pipeline {
            isf_pipeline.encode_render_pass(
                device,
                encoder,
                texture_view,
                self.program_store.isf_time.unwrap(),
            );
        }

        // get render pipeline for current pass
        let render_pipeline = match self.program_store.current_pipeline() {
            Some(pipeline) => pipeline,
            None => return,
        };

        // update GPU data
        self.program_store.update_uniform_buffers(device, encoder);

        // configure pipeline
        let mut render_pass = wgpu::RenderPassBuilder::new()
            .color_attachment(texture_view, |color| color)
            .begin(encoder);

        render_pass.set_pipeline(render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        // attach appropriate bind groups for the current program
        let bind_groups = match self.program_store.get_bind_groups() {
            Some(g) => g,
            None => return,
        };
        for (set, bind_group) in bind_groups.iter().enumerate() {
            render_pass.set_bind_group(set as u32, bind_group, &[]);
        }

        // render quad
        let vertex_range = 0..quad_2d::VERTICES.len() as u32;
        let instance_range = 0..1;
        render_pass.draw(vertex_range, instance_range);
    }

    /// Encode a render pass for each pass.
    pub fn encode_render_passes(&mut self, window: &Ref<'_, Window>, device: &wgpu::Device) {
        self.program_store.reset_pass_index();
        let mut passes = self.program_store.num_passes();
        if passes == 0 {
            passes = 1;
        }

        // encode a render pass for each pass of the shader
        for i in 0..passes {
            // setup environment
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("rusty_vision_render_pass"),
            };
            let mut encoder = device.create_command_encoder(&desc);

            // draw to model texture
            let render_texture = self.program_store.get_render_texture(i as usize);
            let texture_view = render_texture.view().build();
            self.encode_render_pass(device, &mut encoder, &texture_view);

            // copy image into pass texture
            let pass_texture = self.program_store.multipass_textures()[i as usize];
            util::copy_texture(&mut encoder, &render_texture, pass_texture);

            // finish pass
            window.swap_chain_queue().submit(vec![encoder.finish()]);
            self.program_store.increment_pass_index();
        }
    }

    pub fn render_texture_to_frame(&self, frame: &Frame) {
        let mut encoder = frame.command_encoder();
        let texture_reshaper = self.program_store.get_texture_reshaper();
        texture_reshaper.encode_render_pass(frame.texture_view(), &mut *encoder);
    }
}
