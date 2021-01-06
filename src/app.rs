use nannou::prelude::*;
use nannou::ui::prelude::*;
use notify::DebouncedEvent;
use shaderc;
use std::collections::HashMap;
use std::sync::mpsc::Receiver;

use crate::pipelines;
use crate::uniforms;

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
        compilation_errors_wrapper,
        compilation_errors_title,
        compilation_errors_message,
    }
}

pub type CompilationErrors = HashMap<String, shaderc::Error>;

/**
 * Main application state
 */
#[allow(dead_code)] // needed for shader_watcher
pub struct Model {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub compilation_errors: CompilationErrors,
    pub current_program: usize,
    pub widget_ids: WidgetIds,
    pub main_window_id: WindowId,
    pub pipelines: pipelines::Pipelines,
    pub shader_channel: Receiver<DebouncedEvent>,
    pub shader_watcher: notify::FsEventWatcher,
    pub show_controls: bool,
    pub ui: Ui,
    pub ui_show_general: bool,
    pub ui_show_geometry: bool,
    pub uniforms: uniforms::Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
}
