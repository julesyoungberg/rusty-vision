use nannou::prelude::*;

mod app;
mod app_config;
mod interface;
mod programs;
mod quad_2d;
mod util;

fn main() {
    nannou::app(model).update(update).run();
}

/**
 * App setup
 */
fn model(app: &App) -> app::Model {
    // create window
    let main_window_id = app
        .new_window()
        .size(app_config::SIZE[0], app_config::SIZE[1])
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(main_window_id).unwrap();
    let device = window.swap_chain_device();
    let msaa_samples = window.msaa_samples();

    let mut program_store = programs::ProgramStore::new(device);
    program_store.compile_current(device, msaa_samples);
    let vertex_buffer = quad_2d::create_vertex_buffer(device);

    // create UI
    let mut ui = app.new_ui().build().unwrap();
    let widget_ids = app::WidgetIds::new(ui.widget_id_generator());

    app::Model {
        widget_ids,
        main_window_id,
        program_store,
        show_controls: true,
        ui,
        ui_show_color: false,
        ui_show_geometry: true,
        vertex_buffer,
    }
}

/**
 * Update app state
 */
fn update(app: &App, model: &mut app::Model, _update: Update) {
    let window = app.window(model.main_window_id).unwrap();
    let device = window.swap_chain_device();
    model
        .program_store
        .update_shaders(device, window.msaa_samples());

    model.program_store.update_uniforms();

    interface::update(model);
}

/**
 * Handle key pressed event
 */
fn key_pressed(_app: &App, model: &mut app::Model, key: Key) {
    if !model.program_store.current_subscriptions.camera {
        // currently no uniforms other than camera use keys
        return;
    }

    let scale = 0.2;
    let theta = 0.01;

    let camera = &mut model.program_store.buffer_store.camera_uniforms;
    let camera_dir = camera.dir();
    let camera_up = camera.up();
    let cross = camera_dir.cross(camera_up);
    let cross_dir = util::normalize_vector(cross);

    match key {
        Key::H => model.show_controls = !model.show_controls,
        Key::Up => camera.translate(camera_dir * scale),
        Key::Down => camera.translate(camera_dir * -scale),
        Key::Left => camera.translate(cross_dir * -scale),
        Key::Right => camera.translate(cross_dir * scale),
        Key::W => camera.rotate(util::rotate_around_axis(cross_dir, theta)),
        Key::S => camera.rotate(util::rotate_around_axis(cross_dir, -theta)),
        Key::A => camera.rotate(util::rotate_around_axis(camera_up, theta)),
        Key::D => camera.rotate(util::rotate_around_axis(camera_up, -theta)),
        _ => (),
    }
}

/**
 * Draw the state of the app to the frame
 */
fn draw(model: &app::Model, frame: &Frame) -> bool {
    // setup environment
    let device = frame.device_queue_pair().device();
    let render_pipeline = match model.program_store.current_pipeline() {
        Some(pipeline) => pipeline,
        None => return false,
    };
    let mut encoder = frame.command_encoder();

    // send new uniform data to the GPU buffers
    model
        .program_store
        .update_uniform_buffers(device, &mut encoder);

    // configure pipeline
    let mut render_pass = wgpu::RenderPassBuilder::new()
        .color_attachment(&frame.texture_view(), |color| color)
        .begin(&mut encoder);
    render_pass.set_pipeline(&render_pipeline);
    render_pass.set_vertex_buffer(0, &model.vertex_buffer, 0, 0);

    // attach appropriate bind groups for the current program
    let bind_groups = model.program_store.get_bind_groups();
    for (set, bind_group) in bind_groups.iter().enumerate() {
        render_pass.set_bind_group(set as u32, bind_group, &[]);
    }

    // render quad
    let vertex_range = 0..quad_2d::VERTICES.len() as u32;
    let instance_range = 0..1;
    render_pass.draw(vertex_range, instance_range);
    true
}

/**
 * Render app
 */
fn view(app: &App, model: &app::Model, frame: Frame) {
    if model.program_store.programs.keys().len() == 0 || !draw(model, &frame) {
        let draw = app.draw();
        draw.background().color(DARKGRAY);
        draw.to_frame(app, &frame).unwrap();
    }

    if model.show_controls {
        interface::draw(app, model, &frame);
    }
}
