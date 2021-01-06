use nannou::ui::prelude::*;

use crate::app;
use crate::config;
use crate::uniforms;

#[path = "./components.rs"]
mod components;

/**
 * General controls
 */
fn general_conrols(
    widget_ids: &app::WidgetIds,
    uniforms: &mut uniforms::Uniforms,
    ui: &mut UiCell,
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

/**
 * Geometry controls
 */
fn geometry_controls(
    widget_ids: &app::WidgetIds,
    uniforms: &mut uniforms::Uniforms,
    ui: &mut UiCell,
) {
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

/**
 * Info Box
 */
fn info_box(widget_ids: &app::WidgetIds, uniforms: &mut uniforms::Uniforms, ui: &mut UiCell) {
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

/**
 * Main UI logic / layout
 */
pub fn update_ui(model: &mut app::Model) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();

    ////////////////////////
    // compute height
    let mut height = 130.0;
    if model.ui_show_general {
        height = height + 270.0;
        if model.uniforms.data.color_mode == 0 {
            height = height + 100.0;
        }
    }
    if model.ui_show_geometry {
        height = height + 150.0;
    }
    let border = 40.0;
    let scroll = height > config::SIZE[1] as f32 - border;
    if scroll {
        height = config::SIZE[1] as f32 - border;
    }

    /////////////////////////
    // controls wrapper
    let mut controls_wrapper =
        components::container([219.0, height as f64]).top_left_with_margin(10.0);
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
        for selected in components::drop_down(config::PROGRAMS, model.current_program)
            .parent(model.widget_ids.controls_wrapper)
            .down(5.0)
            .set(model.widget_ids.current_program, ui)
        {
            if selected != model.current_program {
                println!("program selected: {}", config::PROGRAMS[selected]);
                model.current_program = selected;
                model.uniforms.set_program_defaults(selected);
            }
        }

        general_conrols(&model.widget_ids, &mut model.uniforms, ui);
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
        geometry_controls(&model.widget_ids, &mut model.uniforms, ui);
    }

    info_box(&model.widget_ids, &mut model.uniforms, ui);
}
