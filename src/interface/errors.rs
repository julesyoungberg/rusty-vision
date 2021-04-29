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
    if let Some(config_error) = &program_store.error {
        error_display(&widget_ids, ui, "Config Error", config_error.as_str(), size);
        return;
    }

    let compile_errors = program_store.get_program_errors();
    if let Some(ref c_errors) = compile_errors {
        if !c_errors.is_empty() {
            compilation_errors(&widget_ids, ui, &compile_errors.unwrap(), size);
            return;
        }
    }

    for (error_type, errors) in program_store.get_data_errors() {
        if !errors.is_empty() {
            let msg = errors
                .iter()
                .fold("".to_owned(), |msg, error| format!("{}{}\n", msg, error));

            error_display(
                &widget_ids,
                ui,
                format!("{} Error", error_type).as_str(),
                msg.as_str(),
                size,
            );

            return;
        }
    }
}
