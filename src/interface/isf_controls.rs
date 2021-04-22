use nannou::prelude::*;
use nannou::ui::prelude::*;

use crate::app;
use crate::interface::components;
use crate::programs::isf::data;
use crate::programs::isf::IsfPipeline;

pub fn height(model: &mut app::Model) -> f32 {
    let isf = match &model.program_store.isf_pipeline {
        Some(isf_pipeline) => match &isf_pipeline.isf {
            Some(isf) => isf,
            None => return 0.0,
        },
        None => return 0.0,
    };

    if isf.inputs.is_empty() {
        return 0.0;
    }

    let mut height = 30.0;

    for input in &isf.inputs {
        match &input.ty {
            isf::InputType::Float(_) | isf::InputType::Long(_) => {
                height += 35.0;
            }
            isf::InputType::Point2d(_) => {
                height += 50.0;
            }
            _ => (),
        };
    }

    height
}

pub fn update(
    widget_ids: &app::WidgetIds,
    ui: &mut UiCell,
    isf_pipeline: &mut IsfPipeline,
    size: Point2,
) {
    let isf = match &isf_pipeline.isf {
        Some(isf) => isf,
        None => return,
    };

    if isf.inputs.is_empty() {
        return;
    }

    if let Some(isf_widget_ids) = &isf_pipeline.widget_ids.as_ref() {
        widget::Text::new("ISF Inputs")
            .rgb(0.9, 0.9, 0.9)
            .font_size(18)
            .parent(widget_ids.controls_wrapper)
            .down(10.0)
            .set(widget_ids.isf_inputs_title, ui);

        let data_inputs = isf_pipeline.isf_data.inputs_mut();
        let mut offset = 0.0;

        for input in &isf.inputs {
            let data = data_inputs.get(&input.name).unwrap().clone();

            match (data, &input.ty) {
                (data::IsfInputData::Float(val), isf::InputType::Float(input_config)) => {
                    let widget_id = match isf_widget_ids.get(&input.name) {
                        Some(id) => id,
                        None => continue,
                    };

                    if let Some(value) = components::slider(
                        *val,
                        input_config.min.unwrap_or(0.0),
                        input_config.max.unwrap_or(1.0),
                    )
                    .parent(widget_ids.controls_wrapper)
                    .down(10.0)
                    .left(offset - 200.0)
                    .label(input.name.as_str())
                    .set(*widget_id, ui)
                    {
                        data_inputs.insert(input.name.clone(), data::IsfInputData::Float(value));
                    }

                    offset = 0.0;
                }
                (data::IsfInputData::Long(val), isf::InputType::Long(input_config)) => {
                    let widget_id = match isf_widget_ids.get(&input.name) {
                        Some(id) => id,
                        None => continue,
                    };

                    let min = input_config.min.unwrap_or(0) as f32;
                    let range = input_config.max.unwrap_or(1) as f32 - min;

                    if let Some(value) = components::slider((*val as f32 - min) / range, 0.0, 1.0)
                        .parent(widget_ids.controls_wrapper)
                        .down(10.0)
                        .left(offset - 200.0)
                        .label(input.name.as_str())
                        .set(*widget_id, ui)
                    {
                        data_inputs.insert(
                            input.name.clone(),
                            data::IsfInputData::Long((value * range + min).round() as i32),
                        );
                    }

                    offset = 0.0;
                }
                (data::IsfInputData::Point2d(val), isf::InputType::Point2d(input_config)) => {
                    let min = input_config.min.unwrap_or([0.0, 0.0]);
                    let max = input_config.max.unwrap_or([size[0] * 2.0, size[1] * 2.0]);
                    let mut v = *val;

                    let mut label_name = input.name.clone();
                    label_name.push_str("-label");
                    let mut x_name = input.name.clone();
                    x_name.push_str("-x");
                    let mut y_name = input.name.clone();
                    y_name.push_str("-y");

                    components::label(input.name.as_str())
                        .left(offset - 43.0)
                        .parent(widget_ids.controls_wrapper)
                        .set(*isf_widget_ids.get(&label_name).unwrap(), ui);

                    if let Some(value) = components::x_2d_slider(v[0], min[0], max[0])
                        .parent(widget_ids.controls_wrapper)
                        .set(*isf_widget_ids.get(&x_name).unwrap(), ui)
                    {
                        v[0] = value;
                        data_inputs.insert(
                            input.name.clone(),
                            data::IsfInputData::Point2d(vec2(v[0], v[1])),
                        );
                    }

                    if let Some(value) = components::y_2d_slider(v[1], min[1], max[1])
                        .parent(widget_ids.controls_wrapper)
                        .set(*isf_widget_ids.get(&y_name).unwrap(), ui)
                    {
                        v[1] = value;
                        data_inputs.insert(
                            input.name.clone(),
                            data::IsfInputData::Point2d(vec2(v[0], v[1])),
                        );
                    }

                    offset = 92.0;
                }
                _ => (),
            };
        }
    }
}
