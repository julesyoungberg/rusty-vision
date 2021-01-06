use nannou::ui::prelude::*;

use crate::app;
use crate::config;
use crate::uniforms;

/**
 * UI Components
 */
fn container(dimensions: [f64; 2]) -> widget::BorderedRectangle {
    widget::BorderedRectangle::new(dimensions)
        .rgba(0.9, 0.9, 0.9, 0.7)
        .border_rgb(0.5, 0.5, 0.5)
        .border(1.0)
}

fn text<'a>(text: &'a str) -> widget::Text<'a> {
    widget::Text::new(text).rgb(0.1, 0.1, 0.1).font_size(12)
}

fn text_small<'a>(text: &'a str) -> widget::Text<'a> {
    widget::Text::new(text).rgb(0.1, 0.1, 0.1).font_size(10)
}

fn button_small(active: bool) -> widget::Button<'static, widget::button::Flat> {
    let mut btn_color = 0.0;
    if active {
        btn_color = 0.5;
    }

    widget::Button::new()
        .w_h(30.0, 20.0)
        .rgb(btn_color, btn_color, btn_color)
        .border(0.0)
}

fn button_big() -> widget::Button<'static, widget::button::Flat> {
    widget::Button::new()
        .w_h(200.0, 36.0)
        .rgb(0.1, 0.1, 0.1)
        .label_rgb(1.0, 1.0, 1.0)
        .label_font_size(18)
        .border(0.0)
}

fn drop_down(
    items: &'static [&str],
    selected: usize,
) -> widget::DropDownList<'static, &'static str> {
    widget::DropDownList::new(items, Option::from(selected))
        .w_h(200.0, 27.0)
        .label_font_size(12)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}

fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    widget::Slider::new(val, min, max)
        .w_h(200.0, 27.0)
        .label_font_size(12)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}

fn slider_small(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    widget::Slider::new(val, min, max)
        .w_h(60.0, 20.0)
        .label_font_size(10)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}

fn unit_slider(val: f32) -> widget::Slider<'static, f32> {
    slider_small(val, 0.0, 1.0)
}

fn red_slider(val: f32) -> widget::Slider<'static, f32> {
    unit_slider(val).rgb(0.8, 0.3, 0.3).down(5.0).label("R")
}

fn green_slider(val: f32) -> widget::Slider<'static, f32> {
    unit_slider(val).rgb(0.3, 0.8, 0.3).right(10.0).label("G")
}

fn blue_slider(val: f32) -> widget::Slider<'static, f32> {
    unit_slider(val).rgb(0.3, 0.3, 0.8).right(10.0).label("B")
}

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
    text(&format!("Draw Floor"))
        .parent(widget_ids.controls_wrapper)
        .down(10.0)
        .set(widget_ids.draw_floor_label, ui);
    let draw_floor = uniforms.data.draw_floor == 1;
    for _click in button_small(draw_floor)
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
    for value in slider(uniforms.data.fog_dist, 15.0, 300.0)
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
    text(&format!("Color Mode"))
        .parent(widget_ids.controls_wrapper)
        .down(10.0)
        .set(widget_ids.color_mode_label, ui);
    for selected in drop_down(config::COLOR_MODES, uniforms.data.color_mode as usize)
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
    text(&format!("Color 1"))
        .parent(widget_ids.controls_wrapper)
        .down(10.0)
        .set(widget_ids.color1_label, ui);
    for value in red_slider(uniforms.data.color1_r)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.color1_r, ui)
    {
        uniforms.data.color1_r = value;
    }
    right = step;
    for value in green_slider(uniforms.data.color1_g)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.color1_g, ui)
    {
        uniforms.data.color1_g = value;
    }
    right = right + step;
    for value in blue_slider(uniforms.data.color1_b)
        .parent(widget_ids.controls_wrapper)
        .set(widget_ids.color1_b, ui)
    {
        uniforms.data.color1_b = value;
    }
    right = right + step;

    if uniforms.data.color_mode == 0 {
        /////////////////////////
        // color 2 select
        text(&format!("Color 2"))
            .parent(widget_ids.controls_wrapper)
            .left(right as f64)
            .down(10.0)
            .set(widget_ids.color2_label, ui);
        for value in red_slider(uniforms.data.color2_r)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color2_r, ui)
        {
            uniforms.data.color2_r = value;
        }
        right = step;
        for value in green_slider(uniforms.data.color2_g)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color2_g, ui)
        {
            uniforms.data.color2_g = value;
        }
        right = right + step;
        for value in blue_slider(uniforms.data.color2_b)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color2_b, ui)
        {
            uniforms.data.color2_b = value;
        }
        right = right + step;

        /////////////////////////
        // color 3 select
        text(&format!("Color 3"))
            .parent(widget_ids.controls_wrapper)
            .left(right as f64)
            .down(10.0)
            .set(widget_ids.color3_label, ui);
        for value in red_slider(uniforms.data.color3_r)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color3_r, ui)
        {
            uniforms.data.color3_r = value;
        }
        for value in green_slider(uniforms.data.color3_g)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color3_g, ui)
        {
            uniforms.data.color3_g = value;
        }
        for value in blue_slider(uniforms.data.color3_b)
            .parent(widget_ids.controls_wrapper)
            .set(widget_ids.color3_b, ui)
        {
            uniforms.data.color3_b = value;
        }
    }

    /////////////////////////
    // shape rotation
    let twopi = 360.0;
    text(&format!("Shape Rotation"))
        .parent(widget_ids.controls_wrapper)
        .left(55.0 as f64)
        .down(10.0)
        .set(widget_ids.shape_rotation_label, ui);

    for value in slider_small(uniforms.data.shape_rotation_x, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .rgb(0.3, 0.3, 0.3)
        .down(5.0)
        .label("X")
        .set(widget_ids.shape_rotation_x, ui)
    {
        uniforms.data.shape_rotation_x = value;
    }

    for value in slider_small(uniforms.data.shape_rotation_y, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .rgb(0.3, 0.3, 0.3)
        .right(10.0)
        .label("Y")
        .set(widget_ids.shape_rotation_y, ui)
    {
        uniforms.data.shape_rotation_y = value;
    }

    for value in slider_small(uniforms.data.shape_rotation_z, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .rgb(0.3, 0.3, 0.3)
        .right(10.0)
        .label("Z")
        .set(widget_ids.shape_rotation_z, ui)
    {
        uniforms.data.shape_rotation_z = value;
    }

    /////////////////////////
    // rotation1
    let twopi = 360.0;
    text(&format!("Rotation 1"))
        .parent(widget_ids.controls_wrapper)
        .left(85.0 as f64)
        .down(10.0)
        .set(widget_ids.rotation1_label, ui);

    for value in slider_small(uniforms.data.rotation1_x, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .rgb(0.3, 0.3, 0.3)
        .down(5.0)
        .label("X")
        .set(widget_ids.rotation1_x, ui)
    {
        uniforms.data.rotation1_x = value;
    }

    for value in slider_small(uniforms.data.rotation1_y, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .rgb(0.3, 0.3, 0.3)
        .right(10.0)
        .label("Y")
        .set(widget_ids.rotation1_y, ui)
    {
        uniforms.data.rotation1_y = value;
    }

    for value in slider_small(uniforms.data.rotation1_z, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .rgb(0.3, 0.3, 0.3)
        .right(10.0)
        .label("Z")
        .set(widget_ids.rotation1_z, ui)
    {
        uniforms.data.rotation1_z = value;
    }

    /////////////////////////
    // rotation2
    let twopi = 360.0;
    text(&format!("Rotation 2"))
        .parent(widget_ids.controls_wrapper)
        .left(85.0 as f64)
        .down(10.0)
        .set(widget_ids.rotation2_label, ui);

    for value in slider_small(uniforms.data.rotation2_x, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .rgb(0.3, 0.3, 0.3)
        .down(5.0)
        .label("X")
        .set(widget_ids.rotation2_x, ui)
    {
        uniforms.data.rotation2_x = value;
    }

    for value in slider_small(uniforms.data.rotation2_y, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .rgb(0.3, 0.3, 0.3)
        .right(10.0)
        .label("Y")
        .set(widget_ids.rotation2_y, ui)
    {
        uniforms.data.rotation2_y = value;
    }

    for value in slider_small(uniforms.data.rotation2_z, 0.0, twopi)
        .parent(widget_ids.controls_wrapper)
        .rgb(0.3, 0.3, 0.3)
        .right(10.0)
        .label("Z")
        .set(widget_ids.rotation2_z, ui)
    {
        uniforms.data.rotation2_z = value;
    }

    /////////////////////////
    // offset1
    let offset_max = 10.0;
    text(&format!("Offset 1"))
        .parent(widget_ids.controls_wrapper)
        .left(100.0 as f64)
        .down(10.0)
        .set(widget_ids.offset1_label, ui);

    for value in slider_small(uniforms.data.offset1_x, 0.0, offset_max)
        .parent(widget_ids.controls_wrapper)
        .rgb(0.3, 0.3, 0.3)
        .down(5.0)
        .label("X")
        .set(widget_ids.offset1_x, ui)
    {
        uniforms.data.offset1_x = value;
    }

    for value in slider_small(uniforms.data.offset1_y, 0.0, offset_max)
        .parent(widget_ids.controls_wrapper)
        .rgb(0.3, 0.3, 0.3)
        .right(10.0)
        .label("Y")
        .set(widget_ids.offset1_y, ui)
    {
        uniforms.data.offset1_y = value;
    }

    for value in slider_small(uniforms.data.offset1_z, 0.0, offset_max)
        .parent(widget_ids.controls_wrapper)
        .rgb(0.3, 0.3, 0.3)
        .right(10.0)
        .label("Z")
        .set(widget_ids.offset1_z, ui)
    {
        uniforms.data.offset1_z = value;
    }
}

/**
 * Info Box
 */
fn info_box(widget_ids: &app::WidgetIds, uniforms: &mut uniforms::Uniforms, ui: &mut UiCell) {
    container([250.0, 80.0])
        .no_parent()
        .top_right_with_margin(10.0)
        .set(widget_ids.info_wrapper, ui);

    text(&format!(
        "Camera Position: <{:.2}, {:.2}, {:.2}>",
        uniforms.data.camera_pos_x, uniforms.data.camera_pos_y, uniforms.data.camera_pos_z
    ))
    .parent(widget_ids.info_wrapper)
    .top_left_with_margin(10.0)
    .set(widget_ids.camera_pos_display, ui);

    text(&format!(
        "Camera Target: <{:.2}, {:.2}, {:.2}>",
        uniforms.data.camera_target_x, uniforms.data.camera_target_y, uniforms.data.camera_target_z
    ))
    .parent(widget_ids.info_wrapper)
    .down(10.0)
    .set(widget_ids.camera_target_display, ui);

    text(&format!(
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
    let mut height = 80.0;
    if model.ui_show_general {
        height = height + 410.0;

        if model.uniforms.data.color_mode == 0 {
            height = height + 90.0;
        }
    }
    let border = 40.0;
    let scroll = height > config::SIZE[1] as f32 - border;
    if scroll {
        height = config::SIZE[1] as f32 - border;
    }

    /////////////////////////
    // controls wrapper
    let mut controls_wrapper = container([219.0, height as f64]).top_left_with_margin(10.0);
    if scroll {
        controls_wrapper = controls_wrapper.scroll_kids_vertically();
    }
    controls_wrapper.set(model.widget_ids.controls_wrapper, ui);

    /////////////////////////
    // hint
    text_small(&format!("Press 'h' to hide"))
        .parent(model.widget_ids.controls_wrapper)
        .top_left_with_margin(10.0)
        .set(model.widget_ids.toggle_controls_hint, ui);

    /////////////////////////
    // general controls tab
    for _click in button_big()
        .parent(model.widget_ids.controls_wrapper)
        .down(10.0)
        .label("General")
        .set(model.widget_ids.general_folder, ui)
    {
        println!("toggle general controls");
        model.ui_show_general = !model.ui_show_general;
    }

    //////////////////////////////////////////////////
    // General Controls
    //////////////////////////////////////////////////
    if model.ui_show_general {
        /////////////////////////
        // current program select
        text(&format!("Shader"))
            .parent(model.widget_ids.controls_wrapper)
            .down(10.0)
            .set(model.widget_ids.current_program_label, ui);
        for selected in drop_down(config::PROGRAMS, model.current_program)
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
    }

    info_box(&model.widget_ids, &mut model.uniforms, ui);
}
