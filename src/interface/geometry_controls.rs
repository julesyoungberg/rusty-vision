use nannou::ui::prelude::*;

use crate::app;
use crate::interface::components;
use crate::programs::uniforms;

/// Section height, computes and returns the current height.
/// Used to compute the container height.
pub fn height(model: &mut app::Model) -> f32 {
    let mut h = 0.0;

    if model.ui_show_geometry {
        h = 260.0;
    }

    h
}

/// Section update, defines layout and update logic of the section
pub fn update(
    widget_ids: &app::WidgetIds,
    ui: &mut UiCell,
    uniforms: &mut uniforms::geometry::GeometryUniforms,
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
    if let Some(value) = components::slider(uniforms.data.fog_dist, 15.0, 300.0)
        .parent(widget_ids.controls_wrapper)
        .left(-30.0)
        .down(10.0)
        .label("Fog Distance")
        .set(widget_ids.fog_dist, ui)
    {
        uniforms.data.fog_dist = value;
    }

    /////////////////////////
    // rotation1
    let twopi = 360.0;
    components::label("Rotation 1")
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.rotation1_label, ui);
    if let Some(value) = components::x_slider(uniforms.data.rotation1_x, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.rotation1_x, ui)
    {
        uniforms.data.rotation1_x = value;
    }
    if let Some(value) = components::y_slider(uniforms.data.rotation1_y, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.rotation1_y, ui)
    {
        uniforms.data.rotation1_y = value;
    }
    if let Some(value) = components::z_slider(uniforms.data.rotation1_z, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.rotation1_z, ui)
    {
        uniforms.data.rotation1_z = value;
    }

    /////////////////////////
    // rotation2
    let twopi = 360.0;
    components::label("Rotation 2")
        .parent(widget_ids.controls_wrapper)
        .left(85.0_f64)
        .set(widget_ids.rotation2_label, ui);
    if let Some(value) = components::x_slider(uniforms.data.rotation2_x, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.rotation2_x, ui)
    {
        uniforms.data.rotation2_x = value;
    }
    if let Some(value) = components::y_slider(uniforms.data.rotation2_y, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.rotation2_y, ui)
    {
        uniforms.data.rotation2_y = value;
    }
    if let Some(value) = components::z_slider(uniforms.data.rotation2_z, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.rotation2_z, ui)
    {
        uniforms.data.rotation2_z = value;
    }

    /////////////////////////
    // offset1
    let offset_max = 10.0;
    components::label("Offset 1")
        .parent(widget_ids.controls_wrapper)
        .left(100.0_f64)
        .set(widget_ids.offset1_label, ui);
    if let Some(value) = components::x_slider(uniforms.data.offset1_x, 0.0, offset_max)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.offset1_x, ui)
    {
        uniforms.data.offset1_x = value;
    }
    if let Some(value) = components::y_slider(uniforms.data.offset1_y, 0.0, offset_max)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.offset1_y, ui)
    {
        uniforms.data.offset1_y = value;
    }
    if let Some(value) = components::z_slider(uniforms.data.offset1_z, 0.0, offset_max)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.offset1_z, ui)
    {
        uniforms.data.offset1_z = value;
    }

    /////////////////////////
    // shape rotation
    let twopi = 360.0;
    components::label("Shape Rotation")
        .parent(widget_ids.controls_wrapper)
        .left(55.0_f64)
        .set(widget_ids.shape_rotation_label, ui);
    if let Some(value) = components::x_slider(uniforms.data.shape_rotation_x, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.shape_rotation_x, ui)
    {
        uniforms.data.shape_rotation_x = value;
    }
    if let Some(value) = components::y_slider(uniforms.data.shape_rotation_y, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.shape_rotation_y, ui)
    {
        uniforms.data.shape_rotation_y = value;
    }
    if let Some(value) = components::z_slider(uniforms.data.shape_rotation_z, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.shape_rotation_z, ui)
    {
        uniforms.data.shape_rotation_z = value;
    }
}
