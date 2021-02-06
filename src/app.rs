use nannou::prelude::*;
use nannou::ui::prelude::*;

use crate::programs;

widget_ids! {
    /**
     * UI widget ids
     */
    pub struct WidgetIds {
        color_mode,
        color_mode_label,
        current_program,
        current_program_label,
        draw_floor,
        draw_floor_label,
        fog_dist,
        general_folder,
        geometry_folder,
        audio_folder,
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
    }
}

/**
 * Main application state
 */
#[allow(dead_code)] // needed for shader_watcher
pub struct Model {
    pub widget_ids: WidgetIds,
    pub main_window_id: WindowId,
    pub program_store: programs::ProgramStore,
    pub show_controls: bool,
    pub ui: Ui,
    pub ui_show_color: bool,
    pub ui_show_geometry: bool,
    pub ui_show_audio: bool,
    pub ui_show_noise: bool,
    pub vertex_buffer: wgpu::Buffer,
}
