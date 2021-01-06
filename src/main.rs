use nannou::prelude::*;

mod app;
mod config;
mod d2;
mod interface;
mod pipelines;
mod shader_store;
mod shaders;
mod uniforms;
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
        .size(config::SIZE[0], config::SIZE[1])
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(main_window_id).unwrap();
    let device = window.swap_chain_device();
    let msaa_samples = window.msaa_samples();

    let mut shader_store = shader_store::ShaderStore::new(device);
    shader_store.compile_shaders(device, msaa_samples);
    let vertex_buffer = d2::create_vertex_buffer(device);

    // create UI
    let mut ui = app.new_ui().build().unwrap();
    let widget_ids = app::WidgetIds::new(ui.widget_id_generator());

    app::Model {
        widget_ids,
        main_window_id,
        shader_store,
        show_controls: true,
        ui,
        ui_show_general: false,
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
        .shader_store
        .update_shaders(device, window.msaa_samples());

    model.shader_store.update_uniforms();

    interface::update(model);
}

/**
 * Handle key pressed event
 */
fn key_pressed(_app: &App, model: &mut app::Model, key: Key) {
    let scale = 0.2;
    let theta = 0.002;

    let camera_dir = model.shader_store.uniforms.camera_dir();
    let camera_up = model.shader_store.uniforms.camera_up();
    let cross = camera_dir.cross(camera_up);
    let cross_dir = util::normalize_vector(cross);

    match key {
        Key::H => model.show_controls = !model.show_controls,
        Key::Up => model
            .shader_store
            .uniforms
            .translate_camera(camera_dir * scale),
        Key::Down => model
            .shader_store
            .uniforms
            .translate_camera(camera_dir * -scale),
        Key::Left => model
            .shader_store
            .uniforms
            .translate_camera(cross_dir * -scale),
        Key::Right => model
            .shader_store
            .uniforms
            .translate_camera(cross_dir * scale),
        Key::W => model
            .shader_store
            .uniforms
            .rotate_camera(util::rotate_around_axis(cross_dir, theta)),
        Key::S => model
            .shader_store
            .uniforms
            .rotate_camera(util::rotate_around_axis(cross_dir, -theta)),
        Key::A => model
            .shader_store
            .uniforms
            .rotate_camera(util::rotate_around_axis(camera_up, theta)),
        Key::D => model
            .shader_store
            .uniforms
            .rotate_camera(util::rotate_around_axis(camera_up, -theta)),
        _ => (),
    }
}

/**
 * Draw the state of the app to the frame
 */
fn draw(model: &app::Model, frame: &Frame) -> bool {
    // setup environment
    let device = frame.device_queue_pair().device();
    let render_pipeline = match model.shader_store.current_pipeline() {
        Some(pipeline) => pipeline,
        None => return false,
    };
    let mut encoder = frame.command_encoder();

    // update uniform buffer
    let uniforms_size = std::mem::size_of::<uniforms::Data>() as wgpu::BufferAddress;
    let uniforms_bytes = model.shader_store.uniforms.as_bytes();
    let uniforms_usage = wgpu::BufferUsage::COPY_SRC;
    let new_uniform_buffer = device.create_buffer_with_data(uniforms_bytes, uniforms_usage);
    encoder.copy_buffer_to_buffer(
        &new_uniform_buffer,
        0,
        &model.shader_store.uniform_buffer,
        0,
        uniforms_size,
    );

    // configure pipeline
    let mut render_pass = wgpu::RenderPassBuilder::new()
        .color_attachment(&frame.texture_view(), |color| color)
        .begin(&mut encoder);
    render_pass.set_pipeline(&render_pipeline);
    render_pass.set_vertex_buffer(0, &model.vertex_buffer, 0, 0);
    render_pass.set_bind_group(0, &model.shader_store.bind_group, &[]);

    // render
    let vertex_range = 0..d2::VERTICES.len() as u32;
    let instance_range = 0..1;
    render_pass.draw(vertex_range, instance_range);
    true
}

/**
 * Render app
 */
fn view(app: &App, model: &app::Model, frame: Frame) {
    if model.shader_store.pipelines.keys().len() == 0 || !draw(model, &frame) {
        let draw = app.draw();
        draw.background().color(DARKGRAY);
        draw.to_frame(app, &frame).unwrap();
    }

    if model.show_controls {
        interface::draw(app, model, &frame);
    }
}
