use nannou::ui::prelude::*;

use crate::app;
use crate::config;
use crate::interface::components;
use crate::programs::uniforms;

/**
 * Section height, computes and returns the current height.
 * Used to compute the container height.
 */
pub fn height(model: &mut app::Model) -> f32 {
    let mut h = 0.0;

    if model.ui_show_general {
        h = 270.0;

        if model
            .program_store
            .buffer_store
            .general_uniforms
            .data
            .color_mode
            == 0
        {
            h = h + 100.0;
        }
    }

    h
}

/**
 * Section update, defines layout and update logic of the section
 */
pub fn update(
    widget_ids: &app::WidgetIds,
    ui: &mut UiCell,
    uniforms: &mut uniforms::general::Uniforms,
) {
    /////////////////////////
    // draw floor toggle
    components::label("Draw Floor")
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.draw_floor_label, ui);
    let draw_floor = uniforms.data.draw_floor == 1;
    for _click in components::button_small(draw_floor)
        .parent(widget_ids.controls_wrapper)
        .right(110.0)
        .set(widget_ids.draw_floor, ui)
    {
        if draw_floor {
            uniforms.data.draw_floor = 0;
        } else {
            uniforms.data.draw_floor = 1;
        }
        println!("draw floor: {}", uniforms.data.draw_floor);
    }

    /////////////////////////
    // fog control
    for value in components::slider(uniforms.data.fog_dist, 15.0, 300.0)
        .parent(widget_ids.controls_wrapper)
        .left(-30.0)
        .down(10.0)
        .label("Fog Distance")
        .set(widget_ids.fog_dist, ui)
    {
        uniforms.data.fog_dist = value;
    }

    /////////////////////////
    // color mode select
    components::label("Color Mode")
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.color_mode_label, ui);
    for selected in components::drop_down(config::COLOR_MODES, uniforms.data.color_mode as usize)
        .parent(widget_ids.controls_wrapper)
        .down(5.0)
        .set(widget_ids.color_mode, ui)
    {
        if selected as i32 != uniforms.data.color_mode {
            println!("color mode selected: {}", config::COLOR_MODES[selected]);
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
    for value in components::red_slider(uniforms.data.color1_r)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.color1_r, ui)
    {
        uniforms.data.color1_r = value;
    }
    right = step;
    for value in components::green_slider(uniforms.data.color1_g)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.color1_g, ui)
    {
        uniforms.data.color1_g = value;
    }
    right = right + step;
    for value in components::blue_slider(uniforms.data.color1_b)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.color1_b, ui)
    {
        uniforms.data.color1_b = value;
    }
    right = right + step;

    if uniforms.data.color_mode == 0 {
        /////////////////////////
        // color 2 select
        components::label("Color 2")
            .parent(widget_ids.controls_wrapper)
            .left(right as f64)
            .set(widget_ids.color2_label, ui);
        for value in components::red_slider(uniforms.data.color2_r)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color2_r, ui)
        {
            uniforms.data.color2_r = value;
        }
        right = step;
        for value in components::green_slider(uniforms.data.color2_g)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color2_g, ui)
        {
            uniforms.data.color2_g = value;
        }
        right = right + step;
        for value in components::blue_slider(uniforms.data.color2_b)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color2_b, ui)
        {
            uniforms.data.color2_b = value;
        }
        right = right + step;

        /////////////////////////
        // color 3 select
        components::label("Color 3")
            .parent(widget_ids.controls_wrapper)
            .left(right as f64)
            .set(widget_ids.color3_label, ui);
        for value in components::red_slider(uniforms.data.color3_r)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color3_r, ui)
        {
            uniforms.data.color3_r = value;
        }
        for value in components::green_slider(uniforms.data.color3_g)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color3_g, ui)
        {
            uniforms.data.color3_g = value;
        }
        for value in components::blue_slider(uniforms.data.color3_b)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color3_b, ui)
        {
            uniforms.data.color3_b = value;
        }
    }

    /////////////////////////
    // shape rotation
    let twopi = 360.0;
    components::label("Shape Rotation")
        .parent(widget_ids.controls_wrapper)
        .left(55.0 as f64)
        .set(widget_ids.shape_rotation_label, ui);
    for value in components::x_slider(uniforms.data.shape_rotation_x, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.shape_rotation_x, ui)
    {
        uniforms.data.shape_rotation_x = value;
    }
    for value in components::y_slider(uniforms.data.shape_rotation_y, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.shape_rotation_y, ui)
    {
        uniforms.data.shape_rotation_y = value;
    }
    for value in components::z_slider(uniforms.data.shape_rotation_z, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.shape_rotation_z, ui)
    {
        uniforms.data.shape_rotation_z = value;
    }
}
