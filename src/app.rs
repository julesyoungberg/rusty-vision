use nannou::prelude::*;
use nannou::ui::prelude::*;
use std::cell::Ref;

use crate::interface;
use crate::programs;
use crate::quad_2d;

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
        audio_features_folder,
        audio_fft_folder,
        general_folder,
        geometry_folder,
        image_folder,
        noise_folder,
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
        fps_container,
        fps,
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
    pub ui: Ui,
    pub ui_show_audio_features: bool,
    pub ui_show_audio_fft: bool,
    pub ui_show_color: bool,
    pub ui_show_geometry: bool,
    pub ui_show_image: bool,
    pub ui_show_noise: bool,
    pub size: Vector2,
    pub vertex_buffer: wgpu::Buffer,
}

impl Model {
    pub fn encode_update(
        &mut self,
        app: &App,
        window: &Ref<'_, Window>,
        device: &wgpu::Device,
        num_samples: u32,
    ) {
        let desc = wgpu::CommandEncoderDescriptor {
            label: Some("nannou_isf_pipeline_update"),
        };
        let mut encoder = device.create_command_encoder(&desc);

        if self.show_controls {
            interface::update(app, device, &mut encoder, self, num_samples);
        }

        self.program_store
            .update_uniforms(device, &mut encoder, self.size, num_samples);

        self.program_store
            .update_shaders(app, device, &mut encoder, num_samples, self.size);

        window.swap_chain_queue().submit(&[encoder.finish()]);
    }

    pub fn encode_render_pass(
        &self,
        texture_view: wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        pipeline: &wgpu::RenderPipeline,
    ) -> bool {
        // configure pipeline
        let mut render_pass = wgpu::RenderPassBuilder::new()
            .color_attachment(&texture_view, |color| color)
            .begin(encoder);

        render_pass.set_pipeline(pipeline);
        render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);

        // attach appropriate bind groups for the current program
        let bind_groups = match self.program_store.get_bind_groups() {
            Some(g) => g,
            None => return false,
        };
        for (set, bind_group) in bind_groups.iter().enumerate() {
            render_pass.set_bind_group(set as u32, bind_group, &[]);
        }

        // render quad
        let vertex_range = 0..quad_2d::VERTICES.len() as u32;
        let instance_range = 0..1;
        render_pass.draw(vertex_range, instance_range);
        true
    }
}
