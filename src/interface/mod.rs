use nannou::prelude::*;
use nannou::ui::prelude::*;
use nannou::ui::DrawToFrameError;

use crate::app;
use crate::config;

mod color_controls;
mod components;
mod errors;
mod geometry_controls;
mod info_box;

/**
 * Main UI logic / layout
 */
pub fn update(model: &mut app::Model) {
    ////////////////////////
    // compute height
    let mut height = 200.0;
    height = height + color_controls::height(model);
    if model.program_store.current_subscriptions.geometry {
        height = height + geometry_controls::height(model);
    }

    let border = 40.0;
    let scroll = height > config::SIZE[1] as f32 - border;
    if scroll {
        height = config::SIZE[1] as f32 - border;
    }

    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();

    /////////////////////////
    // controls wrapper
    let mut controls_wrapper =
        components::container([220.0, height as f64]).top_left_with_margin(10.0);
    if scroll {
        controls_wrapper = controls_wrapper.scroll_kids_vertically();
    }
    controls_wrapper.set(model.widget_ids.controls_wrapper, ui);

    /////////////////////////
    // hint
    components::text_small(&format!("Press 'h' to hide"))
        .parent(model.widget_ids.controls_wrapper)
        .top_left_with_margin(10.0)
        .set(model.widget_ids.toggle_controls_hint, ui);

    /////////////////////////
    // current program select
    components::label("Shader")
        .parent(model.widget_ids.controls_wrapper)
        .set(model.widget_ids.current_program_label, ui);
    for selected in components::drop_down(config::PROGRAMS, model.program_store.current_program)
        .parent(model.widget_ids.controls_wrapper)
        .down(5.0)
        .set(model.widget_ids.current_program, ui)
    {
        model.program_store.select_program(selected);
    }

    //////////////////////////////////////////////////
    // Color Controls
    //////////////////////////////////////////////////
    for _click in components::button_big()
        .parent(model.widget_ids.controls_wrapper)
        .down(20.0)
        .label("Color")
        .set(model.widget_ids.general_folder, ui)
    {
        println!("toggle general controls");
        model.ui_show_color = !model.ui_show_color;
    }

    let mut geometry_left = -200.0;

    if model.ui_show_color {
        color_controls::update(
            &model.widget_ids,
            ui,
            &mut model.program_store.buffer_store.color_uniforms,
        );
        geometry_left = -60.0;
    }

    //////////////////////////////////////////////////
    // Geometry Controls
    //////////////////////////////////////////////////
    if model.program_store.current_subscriptions.geometry {
        for _click in components::button_big()
            .parent(model.widget_ids.controls_wrapper)
            .down(20.0)
            .left(geometry_left as f64)
            .label("Geometry")
            .set(model.widget_ids.geometry_folder, ui)
        {
            println!("toggle geometry controls");
            model.ui_show_geometry = !model.ui_show_geometry;
        }

        if model.ui_show_geometry {
            geometry_controls::update(
                &model.widget_ids,
                ui,
                &mut model.program_store.buffer_store.geometry_uniforms,
            );
        }
    }

    //////////////////////////////////////////////////
    // Other UI
    //////////////////////////////////////////////////
    if model.program_store.current_subscriptions.camera {
        info_box::update(
            &model.widget_ids,
            ui,
            &mut model.program_store.buffer_store.camera_uniforms,
        );
    }

    //////////////////////////////////////////////////
    // Error Display
    //////////////////////////////////////////////////
    let compile_errors = model.program_store.errors();
    if compile_errors.keys().len() > 0 {
        errors::compilation_errors(&model.widget_ids, ui, &compile_errors);
    } else if let Some(audio_error) = &model.program_store.buffer_store.audio_uniforms.error {
        errors::update(&model.widget_ids, ui, "Audio Error", audio_error.as_str());
    }
}

/**
 * Draw the state of the `Ui` to the frame.
 */
pub fn draw(app: &App, model: &app::Model, frame: &Frame) {
    let color_attachment_desc = frame.color_attachment_descriptor();
    let primitives = model.ui.draw();
    let window = app
        .window(model.main_window_id)
        .ok_or(DrawToFrameError::InvalidWindow)
        .unwrap();
    let mut ui_encoder = frame.command_encoder();
    ui::encode_render_pass(
        &model.ui,
        &window,
        primitives,
        color_attachment_desc,
        &mut *ui_encoder,
    )
    .unwrap();
}
