use nannou::ui::prelude::*;

use crate::app;
use crate::interface::components;
use crate::uniforms;

/**
 * Section update, defines layout and update logic of the section
 */
pub fn update(widget_ids: &app::WidgetIds, ui: &mut UiCell, uniforms: &mut uniforms::Uniforms) {
    components::container([250.0, 80.0])
        .no_parent()
        .top_right_with_margin(10.0)
        .set(widget_ids.info_wrapper, ui);

    components::text(&format!(
        "Camera Position: <{:.2}, {:.2}, {:.2}>",
        uniforms.data.camera_pos_x, uniforms.data.camera_pos_y, uniforms.data.camera_pos_z
    ))
    .parent(widget_ids.info_wrapper)
    .top_left_with_margin(10.0)
    .set(widget_ids.camera_pos_display, ui);

    components::text(&format!(
        "Camera Target: <{:.2}, {:.2}, {:.2}>",
        uniforms.data.camera_target_x, uniforms.data.camera_target_y, uniforms.data.camera_target_z
    ))
    .parent(widget_ids.info_wrapper)
    .down(10.0)
    .set(widget_ids.camera_target_display, ui);

    components::text(&format!(
        "Camera Up: <{:.2}, {:.2}, {:.2}>",
        uniforms.data.camera_up_x, uniforms.data.camera_up_y, uniforms.data.camera_up_z
    ))
    .parent(widget_ids.info_wrapper)
    .down(10.0)
    .set(widget_ids.camera_up_display, ui);
}
