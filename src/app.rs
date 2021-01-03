use nannou::prelude::*;
use nannou::ui::prelude::*;
use notify::DebouncedEvent;
use std::sync::mpsc::Receiver;

use crate::pipelines;
use crate::uniforms;

widget_ids! {
    pub struct WidgetIds {
        color_mode,
        current_program,
        draw_floor,
        fog_dist,
        quality,
        shape_color_r,
        shape_color_g,
        shape_color_b,
        palette_color1_r,
        palette_color1_g,
        palette_color1_b,
        palette_color2_r,
        palette_color2_g,
        palette_color2_b,
        palette_color3_r,
        palette_color3_g,
        palette_color3_b,
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
    pub ui: Ui,
    pub uniforms: uniforms::Uniforms,
    pub uniform_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
}
