use nannou::ui::prelude::*;

use crate::app;
use crate::interface::components;
use crate::programs::uniforms::audio_fft;

/// Section height, computes and returns the current height.
/// Used to compute the container height.
pub fn height(model: &mut app::Model) -> f32 {
    let mut h = 0.0;

    if model.ui_show_audio_fft {
        h = 30.0;
    }

    h
}

/// Section update, defines layout and update logic of the section
pub fn update(
    widget_ids: &app::WidgetIds,
    ui: &mut UiCell,
    uniforms: &mut audio_fft::AudioFftUniforms,
) {
    if let Some(value) = components::slider(uniforms.smoothing, 0.0, 0.999999)
        .parent(widget_ids.controls_wrapper)
        .down(10.0)
        .label("Smoothing")
        .set(widget_ids.audio_fft_smoothing, ui)
    {
        uniforms.smoothing = value;
    }
}
