use nannou::ui::prelude::*;

use crate::app;
use crate::config;

pub fn update_ui(model: &mut app::Model) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();

    let current_program_label = format!("Shader");
    widget::Text::new(&current_program_label)
        .top_left_with_margin(20.0)
        .rgb(0.1, 0.1, 0.1)
        .font_size(14)
        .set(model.widget_ids.current_program_label, ui);

    for selected in widget::DropDownList::new(config::PROGRAMS, Option::from(model.current_program))
        .w_h(200.0, 30.0)
        .label_font_size(12)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .down(10.0)
        .set(model.widget_ids.current_program, ui)
    {
        if selected != model.current_program {
            println!("program selected: {}", config::PROGRAMS[selected]);
            model.current_program = selected;
        }
    }

    let floor_btn_label = format!("Draw Floor");
    widget::Text::new(&floor_btn_label)
        .down(10.0)
        .rgb(0.1, 0.1, 0.1)
        .font_size(14)
        .set(model.widget_ids.draw_floor_label, ui);

    let mut floor_btn_color = 0.3;
    let mut floor_btn_label = 1.0;
    if model.uniforms.data.draw_floor {
        floor_btn_color = 0.7;
        floor_btn_label = 0.0;
    }

    for _click in widget::Button::new()
        .right(100.0)
        .w_h(30.0, 30.0)
        .rgb(floor_btn_color, floor_btn_color, floor_btn_color)
        .label_rgb(floor_btn_label, floor_btn_label, floor_btn_label)
        .border(0.0)
        .set(model.widget_ids.draw_floor, ui)
    {
        model.uniforms.data.draw_floor = !model.uniforms.data.draw_floor;
    }

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    for value in slider(model.uniforms.data.fog_dist, 15.0, 300.0)
        .left(-30.0)
        .down(10.0)
        .label("Fog Distance")
        .set(model.widget_ids.fog_dist, ui)
    {
        model.uniforms.data.fog_dist = value;
    }

    for value in slider(model.uniforms.data.quality, 1.0, 3.0)
        .down(10.0)
        .label("Quality")
        .set(model.widget_ids.quality, ui)
    {
        model.uniforms.data.quality = value;
    }

    let color_mode_label = format!("Color Mode");
    widget::Text::new(&color_mode_label)
        .down(10.0)
        .rgb(0.1, 0.1, 0.1)
        .font_size(14)
        .set(model.widget_ids.color_mode_label, ui);

    for selected in widget::DropDownList::new(
        config::COLOR_MODES,
        Option::from(model.uniforms.data.color_mode as usize),
    )
    .w_h(200.0, 30.0)
    .label_font_size(15)
    .rgb(0.3, 0.3, 0.3)
    .label_rgb(1.0, 1.0, 1.0)
    .border(0.0)
    .down(10.0)
    .label("Color Mode")
    .set(model.widget_ids.color_mode, ui)
    {
        if selected as i32 != model.uniforms.data.color_mode {
            println!("color mode selected: {}", config::COLOR_MODES[selected]);
            model.uniforms.data.color_mode = selected as i32;
        }
    }

    let mut right: f32;
    let step = 31.0;

    let color_mode_label = format!("Color 1");
    widget::Text::new(&color_mode_label)
        .down(10.0)
        .rgb(0.1, 0.1, 0.1)
        .font_size(14)
        .set(model.widget_ids.color1_label, ui);

    for value in widget::Slider::new(model.uniforms.data.color1_r, 0.0, 1.0)
        .w_h(60.0, 30.0)
        .label_font_size(15)
        .rgb(0.8, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .down(10.0)
        .label("R")
        .set(model.widget_ids.color1_r, ui)
    {
        model.uniforms.data.color1_r = value;
    }

    right = step;

    for value in widget::Slider::new(model.uniforms.data.color1_g, 0.0, 1.0)
        .w_h(60.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.8, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .right(10.0)
        .label("G")
        .set(model.widget_ids.color1_g, ui)
    {
        model.uniforms.data.color1_g = value;
    }

    right = right + step;

    for value in widget::Slider::new(model.uniforms.data.color1_b, 0.0, 1.0)
        .w_h(60.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.8)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .right(10.0)
        .label("B")
        .set(model.widget_ids.color1_b, ui)
    {
        model.uniforms.data.color1_b = value;
    }

    right = right + step;

    let color_mode_label = format!("Color 2");
    widget::Text::new(&color_mode_label)
        .left(right as f64)
        .down(10.0)
        .rgb(0.1, 0.1, 0.1)
        .font_size(14)
        .set(model.widget_ids.color2_label, ui);

    for value in widget::Slider::new(model.uniforms.data.color2_r, 0.0, 1.0)
        .w_h(60.0, 30.0)
        .label_font_size(15)
        .rgb(0.8, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .down(10.0)
        .label("R")
        .set(model.widget_ids.color2_r, ui)
    {
        model.uniforms.data.color2_r = value;
    }

    right = step;

    for value in widget::Slider::new(model.uniforms.data.color2_g, 0.0, 1.0)
        .w_h(60.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.8, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .right(10.0)
        .label("G")
        .set(model.widget_ids.color2_g, ui)
    {
        model.uniforms.data.color2_g = value;
    }

    right = right + step;

    for value in widget::Slider::new(model.uniforms.data.color2_b, 0.0, 1.0)
        .w_h(60.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.8)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .right(10.0)
        .label("B")
        .set(model.widget_ids.color2_b, ui)
    {
        model.uniforms.data.color2_b = value;
    }

    right = right + step;

    let color_mode_label = format!("Color 3");
    widget::Text::new(&color_mode_label)
        .left(right as f64)
        .down(10.0)
        .rgb(0.1, 0.1, 0.1)
        .font_size(14)
        .set(model.widget_ids.color3_label, ui);

    for value in widget::Slider::new(model.uniforms.data.color3_r, 0.0, 1.0)
        .w_h(60.0, 30.0)
        .label_font_size(15)
        .rgb(0.8, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .down(10.0)
        .label("R")
        .set(model.widget_ids.color3_r, ui)
    {
        model.uniforms.data.color3_r = value;
    }

    // right = 0.0;

    for value in widget::Slider::new(model.uniforms.data.color3_g, 0.0, 1.0)
        .w_h(60.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.8, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .right(10.0)
        .label("G")
        .set(model.widget_ids.color3_g, ui)
    {
        model.uniforms.data.color3_g = value;
    }

    // right = right + step;

    for value in widget::Slider::new(model.uniforms.data.color3_b, 0.0, 1.0)
        .w_h(60.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.8)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .right(10.0)
        .label("B")
        .set(model.widget_ids.color3_b, ui)
    {
        model.uniforms.data.color3_b = value;
    }

    // right = right + step;
}
