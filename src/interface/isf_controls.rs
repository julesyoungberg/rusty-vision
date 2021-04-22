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
            isf::InputType::Float(_) => {
                height += 35.0;
            }
            _ => (),
        };
    }

    height
}

pub fn update(widget_ids: &app::WidgetIds, ui: &mut UiCell, isf_pipeline: &mut IsfPipeline) {
    let isf = match &isf_pipeline.isf {
        Some(isf) => isf,
        None => return,
    };

    if isf.inputs.is_empty() {
        return;
    }

    widget::Text::new("ISF Inputs")
        .rgb(0.9, 0.9, 0.9)
        .font_size(18)
        .parent(widget_ids.controls_wrapper)
        .down(10.0)
        .set(widget_ids.isf_inputs_title, ui);

    let isf_widget_ids = &isf_pipeline.widget_ids.as_ref().unwrap();
    let data_inputs = isf_pipeline.isf_data.inputs_mut();

    for input in &isf.inputs {
        let data = &data_inputs.get(&input.name).unwrap();
        let widget_id = match isf_widget_ids.get(&input.name) {
            Some(id) => id,
            None => continue,
        };

        match data {
            data::IsfInputData::Float(val) => {
                let input_config = match &input.ty {
                    isf::InputType::Float(c) => c,
                    _ => continue,
                };

                if let Some(value) = components::slider(
                    *val,
                    input_config.min.unwrap_or(0.0),
                    input_config.max.unwrap_or(1.0),
                )
                .parent(widget_ids.controls_wrapper)
                .down(10.0)
                .label(input.name.as_str())
                .set(*widget_id, ui)
                {
                    data_inputs.insert(input.name.clone(), data::IsfInputData::Float(value));
                }
            }
            _ => (),
        };
    }
}
