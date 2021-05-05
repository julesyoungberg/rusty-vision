use nannou::ui::prelude::*;

pub fn container(dimensions: [f64; 2]) -> widget::BorderedRectangle {
    widget::BorderedRectangle::new(dimensions)
        .rgba(0.1, 0.1, 0.1, 1.0)
        .border_rgb(0.5, 0.5, 0.5)
        .border(1.0)
}

pub fn wrapper(dimensions: [f64; 2]) -> widget::BorderedRectangle {
    widget::BorderedRectangle::new(dimensions)
        .rgba(0.1, 0.1, 0.1, 1.0)
        .border(0.0)
}

pub fn text(text: &str) -> widget::Text<'_> {
    widget::Text::new(text).rgb(0.9, 0.9, 0.9).font_size(12)
}

pub fn text_small(text: &str) -> widget::Text<'_> {
    widget::Text::new(text).rgb(0.9, 0.9, 0.9).font_size(10)
}

pub fn label(txt: &str) -> widget::Text<'_> {
    text(txt).down(10.0)
}

pub fn button_small(active: bool) -> widget::Button<'static, widget::button::Flat> {
    let mut btn_color = 0.0;
    if active {
        btn_color = 0.5;
    }

    widget::Button::new()
        .w_h(30.0, 20.0)
        .rgb(btn_color, btn_color, btn_color)
        .border(0.0)
}

pub fn button_small_wide(active: bool) -> widget::Button<'static, widget::button::Flat> {
    let mut btn_color = 0.0;
    if active {
        btn_color = 0.5;
    }

    widget::Button::new()
        .w_h(200.0, 20.0)
        .rgb(btn_color, btn_color, btn_color)
        .border(0.0)
        .label_rgb(1.0, 1.0, 1.0)
        .label_font_size(12)
}

pub fn button() -> widget::Button<'static, widget::button::Flat> {
    widget::Button::new()
        .w_h(200.0, 27.0)
        .rgb(0.1, 0.1, 0.1)
        .label_rgb(1.0, 1.0, 1.0)
        .label_font_size(12)
        .border(0.0)
}

pub fn button_big() -> widget::Button<'static, widget::button::Flat> {
    widget::Button::new()
        .w_h(200.0, 36.0)
        .rgb(0.1, 0.1, 0.1)
        .label_rgb(1.0, 1.0, 1.0)
        .label_font_size(18)
        .border(0.0)
}

pub fn drop_down<'a>(items: &'a [&str], selected: usize) -> widget::DropDownList<'a, &'a str> {
    widget::DropDownList::new(items, Option::from(selected))
        .w_h(200.0, 27.0)
        .label_font_size(12)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}

pub fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    widget::Slider::new(val, min, max)
        .w_h(200.0, 27.0)
        .label_font_size(12)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}

pub fn slider_3d_small(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    widget::Slider::new(val, min, max)
        .w_h(60.0, 20.0)
        .label_font_size(10)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}

pub fn slider_2d_small(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    widget::Slider::new(val, min, max)
        .w_h(95.0, 20.0)
        .label_font_size(10)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}

pub fn slider_4d_small(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    widget::Slider::new(val, min, max)
        .w_h(42.5, 20.0)
        .label_font_size(10)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
}

pub fn unit_slider(val: f32) -> widget::Slider<'static, f32> {
    slider_3d_small(val, 0.0, 1.0)
}

pub fn red_slider(val: f32) -> widget::Slider<'static, f32> {
    unit_slider(val).rgb(0.8, 0.3, 0.3).down(5.0).label("R")
}

pub fn green_slider(val: f32) -> widget::Slider<'static, f32> {
    unit_slider(val).rgb(0.3, 0.8, 0.3).right(10.0).label("G")
}

pub fn blue_slider(val: f32) -> widget::Slider<'static, f32> {
    unit_slider(val).rgb(0.3, 0.3, 0.8).right(10.0).label("B")
}

pub fn x_slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    slider_3d_small(val, min, max)
        .rgb(0.3, 0.3, 0.3)
        .down(5.0)
        .label("X")
}

pub fn y_slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    slider_3d_small(val, min, max)
        .rgb(0.3, 0.3, 0.3)
        .right(10.0)
        .label("Y")
}

pub fn z_slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    slider_3d_small(val, min, max)
        .rgb(0.3, 0.3, 0.3)
        .right(10.0)
        .label("Z")
}

pub fn x_2d_slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    slider_2d_small(val, min, max)
        .rgb(0.3, 0.3, 0.3)
        .down(5.0)
        .label("X")
}

pub fn y_2d_slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    slider_2d_small(val, min, max)
        .rgb(0.3, 0.3, 0.3)
        .right(10.0)
        .label("Y")
}

pub fn r_4d_slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    slider_4d_small(val, min, max)
        .rgb(0.8, 0.3, 0.3)
        .down(5.0)
        .label("R")
}

pub fn g_4d_slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    slider_4d_small(val, min, max)
        .rgb(0.3, 0.8, 0.3)
        .right(10.0)
        .label("G")
}

pub fn b_4d_slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    slider_4d_small(val, min, max)
        .rgb(0.3, 0.3, 0.8)
        .right(10.0)
        .label("B")
}

pub fn a_4d_slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
    slider_4d_small(val, min, max)
        .rgb(0.5, 0.5, 0.5)
        .right(10.0)
        .label("A")
}
