use nannou::ui::prelude::*;

use crate::app;
use crate::interface::components;
use crate::programs::uniforms::color;

const COLOR_MODES: &[&str] = &["palette", "solid"];

/// Section height, computes and returns the current height.
/// Used to compute the container height.
pub fn height(model: &mut app::Model) -> f32 {
    let mut h = 0.0;

    if model.ui_show_color {
        h = 100.0;

        if model
            .program_store
            .buffer_store
            .color_uniforms
            .data
            .color_mode
            == 0
        {
            h += 100.0;
        }
    }

    h
}

/// Section update, defines layout and update logic of the section
pub fn update(widget_ids: &app::WidgetIds, ui: &mut UiCell, uniforms: &mut color::ColorUniforms) {
    /////////////////////////
    // color mode select
    components::label("Color Mode")
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.color_mode_label, ui);
    if let Some(selected) = components::drop_down(COLOR_MODES, uniforms.data.color_mode as usize)
        .parent(widget_ids.controls_wrapper)
        .down(5.0)
        .set(widget_ids.color_mode, ui)
    {
        if selected as i32 != uniforms.data.color_mode {
            println!("color mode selected: {}", COLOR_MODES[selected]);
            uniforms.data.color_mode = selected as i32;
        }
    }

    let mut right: f32;
    let step = 34.0;

    /////////////////////////
    // color 1 select
    components::label("Color 1")
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.color1_label, ui);
    if let Some(value) = components::red_slider(uniforms.data.color1_r)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.color1_r, ui)
    {
        uniforms.data.color1_r = value;
    }
    right = step;
    if let Some(value) = components::green_slider(uniforms.data.color1_g)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.color1_g, ui)
    {
        uniforms.data.color1_g = value;
    }
    right += step;
    if let Some(value) = components::blue_slider(uniforms.data.color1_b)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.color1_b, ui)
    {
        uniforms.data.color1_b = value;
    }
    right += step;

    if uniforms.data.color_mode == 0 {
        /////////////////////////
        // color 2 select
        components::label("Color 2")
            .parent(widget_ids.controls_wrapper)
            .left(right as f64)
            .set(widget_ids.color2_label, ui);
        if let Some(value) = components::red_slider(uniforms.data.color2_r)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color2_r, ui)
        {
            uniforms.data.color2_r = value;
        }
        right = step;
        if let Some(value) = components::green_slider(uniforms.data.color2_g)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color2_g, ui)
        {
            uniforms.data.color2_g = value;
        }
        right += step;
        if let Some(value) = components::blue_slider(uniforms.data.color2_b)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color2_b, ui)
        {
            uniforms.data.color2_b = value;
        }
        right += step;

        /////////////////////////
        // color 3 select
        components::label("Color 3")
            .parent(widget_ids.controls_wrapper)
            .left(right as f64)
            .set(widget_ids.color3_label, ui);
        if let Some(value) = components::red_slider(uniforms.data.color3_r)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color3_r, ui)
        {
            uniforms.data.color3_r = value;
        }
        if let Some(value) = components::green_slider(uniforms.data.color3_g)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color3_g, ui)
        {
            uniforms.data.color3_g = value;
        }
        if let Some(value) = components::blue_slider(uniforms.data.color3_b)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color3_b, ui)
        {
            uniforms.data.color3_b = value;
        }
    }
}
