use nannou::prelude::*;
use nannou::ui::prelude::*;

use crate::app;
use crate::interface::components;
use crate::programs::program::ProgramErrors;
use crate::programs::ProgramStore;

fn error_display(
    widget_ids: &app::WidgetIds,
    ui: &mut UiCell,
    title: &str,
    message: &str,
    size: Vector2,
) {
    let container_id = widget_ids.errors_wrapper;
    let padding = 40.0;
    components::container([(size[0] - padding) as f64, (size[1] - padding) as f64])
        .no_parent()
        .rgba(0.2, 0.2, 0.2, 1.0)
        .x_y(0.0, 0.0)
        .scroll_kids()
        .set(container_id, ui);

    components::text(title)
        .parent(container_id)
        .top_left_with_margin(20.0)
        .font_size(36)
        .rgb(1.0, 0.3, 0.3)
        .set(widget_ids.errors_title, ui);

    components::text(message)
        .parent(container_id)
        .down(20.0)
        .font_size(18)
        .rgb(0.9, 0.9, 0.9)
        .set(widget_ids.errors_message, ui);
}

pub fn compilation_errors(
    widget_ids: &app::WidgetIds,
    ui: &mut UiCell,
    errors: &ProgramErrors,
    size: Vector2,
) {
    let mut error_string: String = "".to_owned();
    for value in errors.values() {
        error_string.push_str(value.as_str());
        error_string.push('\n');
    }

    error_display(
        widget_ids,
        ui,
        "Compilation Errors",
        error_string.as_str(),
        size,
    );
}

pub fn update(
    program_store: &ProgramStore,
    widget_ids: &app::WidgetIds,
    ui: &mut UiCell,
    size: Vector2,
) {
    let compile_errors = program_store.get_program_errors();
    if let Some(config_error) = &program_store.error {
        error_display(&widget_ids, ui, "Config Error", config_error.as_str(), size);
        return;
    }

    if let Some(c_errors) = compile_errors {
        if c_errors.keys().len() > 0 {
            compilation_errors(&widget_ids, ui, &compile_errors.unwrap(), size);
            return;
        }
    }

    if let Some(audio_error) = &program_store.buffer_store.audio_source.error {
        error_display(&widget_ids, ui, "Audio Error", audio_error.as_str(), size);
        return;
    }

    if let Some(audio_error) = &program_store.buffer_store.audio_features_uniforms.error {
        error_display(
            &widget_ids,
            ui,
            "Audio Features Error",
            audio_error.as_str(),
            size,
        );
        return;
    }

    if let Some(image_error) = &program_store.buffer_store.image_uniforms.error {
        error_display(&widget_ids, ui, "Image Error", image_error.as_str(), size);
        return;
    }

    if let Some(capture) = &program_store.buffer_store.video_uniforms.video_capture {
        if let Some(video_error) = &capture.error {
            error_display(&widget_ids, ui, "Video Error", video_error.as_str(), size);
            return;
        }
    }

    if let Some(capture) = &program_store.buffer_store.webcam_uniforms.video_capture {
        if let Some(webcam_error) = &capture.error {
            error_display(&widget_ids, ui, "Webcam Error", webcam_error.as_str(), size);
            return;
        }
    }
}
