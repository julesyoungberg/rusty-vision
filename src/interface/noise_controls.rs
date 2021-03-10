use nannou::ui::prelude::*;

use crate::app;
use crate::interface::components;
use crate::programs::uniforms::noise;

/// Section height, computes and returns the current height.
/// Used to compute the container height.
pub fn height(model: &mut app::Model) -> f32 {
    let mut h = 0.0;

    if model.ui_show_noise {
        h = 220.0;
    }

    h
}

/// Section update, defines layout and update logic of the section
pub fn update(widget_ids: &app::WidgetIds, ui: &mut UiCell, uniforms: &mut noise::NoiseUniforms) {
    /////////////////////////
    // lacunarity
    if let Some(value) = components::slider(uniforms.data.lacunarity, 0.0, 5.0)
        .parent(widget_ids.controls_wrapper)
        .down(10.0)
        .label("Lacunarity")
        .set(widget_ids.noise_lacunarity, ui)
    {
        uniforms.data.lacunarity = value;
    }

    /////////////////////////
    // gain
    if let Some(value) = components::slider(uniforms.data.gain, 0.0, 1.0)
        .parent(widget_ids.controls_wrapper)
        .down(10.0)
        .label("Gain")
        .set(widget_ids.noise_gain, ui)
    {
        uniforms.data.gain = value;
    }

    /////////////////////////
    // speed
    if let Some(value) = components::slider(uniforms.data.speed, 0.0, 0.5)
        .parent(widget_ids.controls_wrapper)
        .down(10.0)
        .label("Speed")
        .set(widget_ids.noise_speed, ui)
    {
        uniforms.data.speed = value;
    }

    /////////////////////////
    // Invert
    components::label("Invert")
        .parent(widget_ids.controls_wrapper)
        .down(10.0)
        .set(widget_ids.noise_invert_label, ui);
    let invert = uniforms.data.invert == 1;
    for _click in components::button_small(invert)
        .parent(widget_ids.controls_wrapper)
        .right(137.0)
        .set(widget_ids.noise_invert, ui)
    {
        if invert {
            uniforms.data.invert = 0;
        } else {
            uniforms.data.invert = 1;
        }
    }

    /////////////////////////
    // Mirror
    components::label("Mirror")
        .parent(widget_ids.controls_wrapper)
        .down(10.0)
        .left(135.0)
        .set(widget_ids.noise_mirror_label, ui);
    let mirror = uniforms.data.mirror == 1;
    for _click in components::button_small(mirror)
        .parent(widget_ids.controls_wrapper)
        .right(135.0)
        .set(widget_ids.noise_mirror, ui)
    {
        if mirror {
            uniforms.data.mirror = 0;
        } else {
            uniforms.data.mirror = 1;
        }
    }

    /////////////////////////
    // Scale By Prev
    components::label("Scale By Prev")
        .parent(widget_ids.controls_wrapper)
        .down(10.0)
        .left(98.0)
        .set(widget_ids.noise_scale_by_prev_label, ui);
    let scale_by_prev = uniforms.data.scale_by_prev == 1;
    for _click in components::button_small(scale_by_prev)
        .parent(widget_ids.controls_wrapper)
        .right(98.0)
        .set(widget_ids.noise_scale_by_prev, ui)
    {
        if scale_by_prev {
            uniforms.data.scale_by_prev = 0;
        } else {
            uniforms.data.scale_by_prev = 1;
        }
    }

    /////////////////////////
    // Sharpen
    components::label("Sharpen")
        .parent(widget_ids.controls_wrapper)
        .down(10.0)
        .left(125.0)
        .set(widget_ids.noise_sharpen_label, ui);
    let sharpen = uniforms.data.sharpen == 1;
    for _click in components::button_small(sharpen)
        .parent(widget_ids.controls_wrapper)
        .right(125.0)
        .set(widget_ids.noise_sharpen, ui)
    {
        if sharpen {
            uniforms.data.sharpen = 0;
        } else {
            uniforms.data.sharpen = 1;
        }
    }
}
