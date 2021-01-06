use nannou::prelude::*;
use nannou::ui::prelude::*;
use nannou::ui::DrawToFrameError;

use crate::app;
use crate::config;

mod compilation_errors;
mod components;
mod general_controls;
mod geometry_controls;
mod info_box;

/**
 * Main UI logic / layout
 */
pub fn update(model: &mut app::Model) {
    ////////////////////////
    // compute height
    let mut height = 130.0;
    height = height + general_controls::height(model);
    height = height + geometry_controls::height(model);
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

    //////////////////////////////////////////////////
    // General Controls
    //////////////////////////////////////////////////
    for _click in components::button_big()
        .parent(model.widget_ids.controls_wrapper)
        .down(10.0)
        .label("General")
        .set(model.widget_ids.general_folder, ui)
    {
        println!("toggle general controls");
        model.ui_show_general = !model.ui_show_general;
    }

    let mut geometry_left = -200.0;

    if model.ui_show_general {
        /////////////////////////
        // current program select
        components::label("Shader")
            .parent(model.widget_ids.controls_wrapper)
            .set(model.widget_ids.current_program_label, ui);
        for selected in components::drop_down(config::PROGRAMS, model.shader_store.current_program)
            .parent(model.widget_ids.controls_wrapper)
            .down(5.0)
            .set(model.widget_ids.current_program, ui)
        {
            if selected != model.shader_store.current_program {
                println!("program selected: {}", config::PROGRAMS[selected]);
                model.shader_store.current_program = selected;
                model.shader_store.uniforms.set_program_defaults(selected);
            }
        }

        general_controls::update(&model.widget_ids, ui, &mut model.shader_store.uniforms);
        geometry_left = -60.0;
    }

    //////////////////////////////////////////////////
    // Geometry Controls
    //////////////////////////////////////////////////
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
        geometry_controls::update(&model.widget_ids, ui, &mut model.shader_store.uniforms);
    }

    //////////////////////////////////////////////////
    // Other UI
    //////////////////////////////////////////////////
    info_box::update(&model.widget_ids, ui, &mut model.shader_store.uniforms);

    if model.shader_store.compilation_errors.keys().len() > 0 {
        compilation_errors::update(
            &model.widget_ids,
            ui,
            &model.shader_store.compilation_errors,
        );
    }
}

/**
 * Draw the state of the `Ui` to the frame.
 */
pub fn draw(app: &App, model: &app::Model, frame: &Frame) {
    // let draw = app.draw();
    // draw.quad().color(STEELBLUE).x_y(0.0, 0.0).w_h(200.0, 200.0);
    // draw.to_frame(app, &frame).unwrap();

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
