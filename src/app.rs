use nannou::prelude::*;
use nannou::ui::prelude::*;
use notify::DebouncedEvent;
use std::sync::mpsc::Receiver;

use crate::pipelines;
use crate::uniforms;

widget_ids! {
    pub struct WidgetIds {
        color_mode,
        color_mode_label,
        current_program,
        current_program_label,
        draw_floor,
        draw_floor_label,
        fog_dist,
        general_folder,
        quality,
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
        controls_rect,
        rotation1_label,
        rotation1_x,
        rotation1_y,
        rotation1_z,
        offset1_label,
        offset1_x,
        offset1_y,
        offset1_z,
    }
}

#[allow(dead_code)]
pub struct Model {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub current_program: usize,
    pub widget_ids: WidgetIds,
    pub main_window_id: WindowId,
    pub pipelines: pipelines::Pipelines,
    pub shader_channel: Receiver<DebouncedEvent>,
    pub shader_watcher: notify::FsEventWatcher,
    pub show_controls: bool,
    pub ui: Ui,
    pub ui_show_general: bool,
    pub uniforms: uniforms::Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
}
