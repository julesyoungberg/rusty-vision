use nannou::ui::prelude::*;

use crate::app;
use crate::interface::components;

/**
 * Section update, defines layout and update logic of the section
 */
pub fn update(widget_ids: &app::WidgetIds, ui: &mut UiCell, errors: &app::CompilationErrors) {
    let container_id = widget_ids.compilation_errors_wrapper;
    components::container([1200.0, 600.0])
        .no_parent()
        .rgba(0.2, 0.2, 0.2, 1.0)
        .x_y(0.0, 0.0)
        .set(container_id, ui);

    components::text("Compilation Errors")
        .parent(container_id)
        .top_left_with_margin(50.0)
        .font_size(36)
        .rgb(1.0, 0.3, 0.3)
        .set(widget_ids.compilation_errors_title, ui);

    let mut error_string: String = "".to_owned();
    for (_key, value) in errors {
        error_string.push_str(value.to_string().as_str());
        error_string.push_str("\n");
    }

    components::text(error_string.as_str())
        .parent(container_id)
        .down(20.0)
        .font_size(18)
        .rgb(0.9, 0.9, 0.9)
        .set(widget_ids.compilation_errors_message, ui);
}
