use nannou::ui::prelude::*;

use crate::app;
use crate::interface::components;
use crate::programs::uniforms;

/**
 * Section height, computes and returns the current height.
 * Used to compute the container height.
 */
pub fn height(model: &mut app::Model) -> f32 {
    let mut h = 0.0;

    if model.ui_show_geometry {
        h = 150.0;
    }

    h
}

/**
 * Section update, defines layout and update logic of the section
 */
pub fn update(widget_ids: &app::WidgetIds, ui: &mut UiCell, uniforms: &mut uniforms::Uniforms) {
    /////////////////////////
    // rotation1
    let twopi = 360.0;
    components::label("Rotation 1")
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.rotation1_label, ui);
    for value in components::x_slider(uniforms.data.rotation1_x, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.rotation1_x, ui)
    {
        uniforms.data.rotation1_x = value;
    }
    for value in components::y_slider(uniforms.data.rotation1_y, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.rotation1_y, ui)
    {
        uniforms.data.rotation1_y = value;
    }
    for value in components::z_slider(uniforms.data.rotation1_z, 0.0, twopi)
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
        .left(85.0 as f64)
        .set(widget_ids.rotation2_label, ui);
    for value in components::x_slider(uniforms.data.rotation2_x, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.rotation2_x, ui)
    {
        uniforms.data.rotation2_x = value;
    }
    for value in components::y_slider(uniforms.data.rotation2_y, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.rotation2_y, ui)
    {
        uniforms.data.rotation2_y = value;
    }
    for value in components::z_slider(uniforms.data.rotation2_z, 0.0, twopi)
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
        .left(100.0 as f64)
        .set(widget_ids.offset1_label, ui);
    for value in components::x_slider(uniforms.data.offset1_x, 0.0, offset_max)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.offset1_x, ui)
    {
        uniforms.data.offset1_x = value;
    }
    for value in components::y_slider(uniforms.data.offset1_y, 0.0, offset_max)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.offset1_y, ui)
    {
        uniforms.data.offset1_y = value;
    }
    for value in components::z_slider(uniforms.data.offset1_z, 0.0, offset_max)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.offset1_z, ui)
    {
        uniforms.data.offset1_z = value;
    }
}
