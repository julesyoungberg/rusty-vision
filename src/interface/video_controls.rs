use nannou::prelude::*;
use nannou::ui::prelude::*;

use crate::app;
use crate::interface::components;
use crate::programs::uniforms::video;

/// Section height, computes and returns the current height.
/// Used to compute the container height.
pub fn height(model: &mut app::Model) -> f32 {
    let mut h = 0.0;

    if model.ui_show_video {
        h = 60.0;
    }

    h
}

/// Section update, defines layout and update logic of the section
pub fn update(
    device: &wgpu::Device,
    widget_ids: &app::WidgetIds,
    ui: &mut UiCell,
    uniforms: &mut video::VideoUniforms,
) {
    let mut label = "Video".to_owned();
    if let Some(video_name) = &uniforms.video_name {
        label.push_str(": ");
        label.push_str(video_name.as_str());
    }

    components::label(label.as_str())
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.video_label, ui);

    for _click in components::button()
        .parent(widget_ids.controls_wrapper)
        .down(10.0)
        .label("Load")
        .set(widget_ids.video_load_button, ui)
    {
        uniforms.select_video(device);
    }
}
