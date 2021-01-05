use nannou::ui::prelude::*;

use crate::app;
use crate::config;

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

fn unit_slider(val: f32) -> widget::Slider<'static, f32> {
    widget::Slider::new(val, 0.0, 1.0)
        .w_h(60.0, 27.0)
        .label_font_size(12)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}

pub fn update_ui(model: &mut app::Model) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();

    widget::BorderedRectangle::new([219.0, 500.0])
        .top_left_with_margin(10.0)
        .rgba(0.9, 0.9, 0.9, 0.7)
        .border_rgb(0.5, 0.5, 0.5)
        .border(1.0)
        .set(model.widget_ids.controls_rect, ui);

    text_small(&format!("Press 'h' to hide controls"))
        .top_left_with_margin(10.0)
        .parent(model.widget_ids.controls_rect)
        .set(model.widget_ids.toggle_controls_hint, ui);

    for _click in button_big()
        .down(10.0)
        .label("General")
        .set(model.widget_ids.general_folder, ui)
    {
        println!("toggle general controls");
        model.ui_show_general = !model.ui_show_general;
    }

    if model.ui_show_general {
        text(&format!("Shader"))
            .down(10.0)
            .set(model.widget_ids.current_program_label, ui);

        for selected in drop_down(config::PROGRAMS, model.current_program)
            .down(5.0)
            .set(model.widget_ids.current_program, ui)
        {
            if selected != model.current_program {
                println!("program selected: {}", config::PROGRAMS[selected]);
                model.current_program = selected;
            }
        }

        text(&format!("Draw Floor"))
            .down(10.0)
            .set(model.widget_ids.draw_floor_label, ui);

        for _click in button_small(model.uniforms.data.draw_floor)
            .right(110.0)
            .set(model.widget_ids.draw_floor, ui)
        {
            model.uniforms.data.draw_floor = !model.uniforms.data.draw_floor;
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

        text(&format!("Color Mode"))
            .down(10.0)
            .set(model.widget_ids.color_mode_label, ui);

        for selected in drop_down(config::COLOR_MODES, model.uniforms.data.color_mode as usize)
            .down(5.0)
            .set(model.widget_ids.color_mode, ui)
        {
            if selected as i32 != model.uniforms.data.color_mode {
                println!("color mode selected: {}", config::COLOR_MODES[selected]);
                model.uniforms.data.color_mode = selected as i32;
            }
        }

        let mut right: f32;
        let step = 34.0;

        text(&format!("Color 1"))
            .down(10.0)
            .set(model.widget_ids.color1_label, ui);

        for value in unit_slider(model.uniforms.data.color1_r)
            .rgb(0.8, 0.3, 0.3)
            .label("R")
            .set(model.widget_ids.color1_r, ui)
        {
            model.uniforms.data.color1_r = value;
        }

        right = step;

        for value in unit_slider(model.uniforms.data.color1_g)
            .rgb(0.3, 0.8, 0.3)
            .right(10.0)
            .label("G")
            .set(model.widget_ids.color1_g, ui)
        {
            model.uniforms.data.color1_g = value;
        }

        right = right + step;

        for value in unit_slider(model.uniforms.data.color1_b)
            .rgb(0.3, 0.3, 0.8)
            .right(10.0)
            .label("B")
            .set(model.widget_ids.color1_b, ui)
        {
            model.uniforms.data.color1_b = value;
        }

        right = right + step;

        text(&format!("Color 2"))
            .left(right as f64)
            .down(10.0)
            .set(model.widget_ids.color2_label, ui);

        for value in unit_slider(model.uniforms.data.color2_r)
            .rgb(0.8, 0.3, 0.3)
            .down(5.0)
            .label("R")
            .set(model.widget_ids.color2_r, ui)
        {
            model.uniforms.data.color2_r = value;
        }

        right = step;

        for value in unit_slider(model.uniforms.data.color2_g)
            .rgb(0.3, 0.8, 0.3)
            .right(10.0)
            .label("G")
            .set(model.widget_ids.color2_g, ui)
        {
            model.uniforms.data.color2_g = value;
        }

        right = right + step;

        for value in unit_slider(model.uniforms.data.color2_b)
            .rgb(0.3, 0.3, 0.8)
            .right(10.0)
            .label("B")
            .set(model.widget_ids.color2_b, ui)
        {
            model.uniforms.data.color2_b = value;
        }

        right = right + step;

        text(&format!("Color 3"))
            .left(right as f64)
            .down(10.0)
            .set(model.widget_ids.color3_label, ui);

        for value in unit_slider(model.uniforms.data.color3_r)
            .rgb(0.8, 0.3, 0.3)
            .down(5.0)
            .label("R")
            .set(model.widget_ids.color3_r, ui)
        {
            model.uniforms.data.color3_r = value;
        }

        // right = 0.0;

        for value in unit_slider(model.uniforms.data.color3_g)
            .rgb(0.3, 0.8, 0.3)
            .right(10.0)
            .label("G")
            .set(model.widget_ids.color3_g, ui)
        {
            model.uniforms.data.color3_g = value;
        }

        // right = right + step;

        for value in unit_slider(model.uniforms.data.color3_b)
            .rgb(0.3, 0.3, 0.8)
            .right(10.0)
            .label("B")
            .set(model.widget_ids.color3_b, ui)
        {
            model.uniforms.data.color3_b = value;
        }

        // right = right + step;
    }
}
