use nannou::ui::prelude::*;

use crate::app;
use crate::config;

pub fn update_ui(model: &mut app::Model) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();

    for selected in widget::DropDownList::new(config::PROGRAMS, Option::from(model.current_program))
        .w_h(200.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .top_left_with_margin(20.0)
        .label("Current Program")
        .set(model.widget_ids.current_program, ui)
    {
        if selected != model.current_program {
            println!("program selected: {}", config::PROGRAMS[selected]);
            model.current_program = selected;
        }
    }

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

    let mut floor_btn_color = 0.3;
    let mut floor_btn_label = 1.0;
    if model.uniforms.data.draw_floor {
        floor_btn_color = 0.7;
        floor_btn_label = 0.0;
    }

    for _click in widget::Button::new()
        .down(10.0)
        .w_h(200.0, 30.0)
        .label_font_size(15)
        .label("Draw Floor")
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

    for value in widget::Slider::new(model.uniforms.data.shape_color_r, 0.0, 1.0)
        .w_h(60.0, 30.0)
        .label_font_size(15)
        .rgb(0.8, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .down(10.0)
        .label("R")
        .set(model.widget_ids.shape_color_r, ui)
    {
        model.uniforms.data.shape_color_r = value;
    }

    for value in widget::Slider::new(model.uniforms.data.shape_color_g, 0.0, 1.0)
        .w_h(60.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.8, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .right(10.0)
        .label("G")
        .set(model.widget_ids.shape_color_g, ui)
    {
        model.uniforms.data.shape_color_g = value;
    }

    for value in widget::Slider::new(model.uniforms.data.shape_color_b, 0.0, 1.0)
        .w_h(60.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.8)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .right(10.0)
        .label("B")
        .set(model.widget_ids.shape_color_b, ui)
    {
        model.uniforms.data.shape_color_b = value;
    }
}
