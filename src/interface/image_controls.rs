use nannou::prelude::*;
use nannou::ui::prelude::*;

use crate::app;
use crate::interface::components;
use crate::programs::uniforms::image;

/**
 * Section height, computes and returns the current height.
 * Used to compute the container height.
 */
pub fn height(model: &mut app::Model) -> f32 {
    let mut h = 0.0;

    if model.ui_show_image {
        h = 50.0;
    }

    h
}

/**
 * Section update, defines layout and update logic of the section
 */
pub fn update(
    app: &App,
    widget_ids: &app::WidgetIds,
    ui: &mut UiCell,
    uniforms: &mut image::ImageUniforms,
) {
    let mut label = "Image 1".to_owned();
    if let Some(image1_path) = &uniforms.image1_path {
        label.push_str(": ");
        label.push_str(image1_path.as_str());
    }

    components::label(label.as_str())
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.image1_label, ui);

    for _click in components::button()
        .parent(widget_ids.controls_wrapper)
        .down(10.0)
        .label("Load")
        .set(widget_ids.image1_load_button, ui)
    {
        uniforms.load_image();
    }
}
