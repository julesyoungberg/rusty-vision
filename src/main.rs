use nannou::prelude::*;
use std::{thread, time};

mod app;
mod interface;
mod programs;
mod quad_2d;
mod util;

fn main() {
    nannou::app(model).update(update).run();
}

/// App setup
fn model(app: &App) -> app::Model {
    // create window
    let main_window_id = app
        .new_window()
        .size(1920, 1080)
        .key_pressed(key_pressed)
        .unfocused(pause)
        .focused(unpause)
        .resizable(true)
        .resized(resized)
        .mouse_moved(mouse_moved)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(main_window_id).unwrap();
    let device = window.swap_chain_device();
    let msaa_samples = window.msaa_samples();

    let (width, height) = window.inner_size_pixels();
    let size = pt2(width as f32, height as f32);
    let mut program_store = programs::ProgramStore::new(app, device, size);
    program_store.configure(app, device, msaa_samples);
    let vertex_buffer = quad_2d::create_vertex_buffer(device);

    // create UI
    let mut ui = app.new_ui().build().unwrap();
    let widget_ids = app::WidgetIds::new(ui.widget_id_generator());

    app::Model {
        widget_ids,
        main_window_id,
        original_height: height,
        original_width: width,
        paused: false,
        program_store,
        show_controls: true,
        ui,
        ui_show_audio_features: false,
        ui_show_audio_fft: false,
        ui_show_color: false,
        ui_show_geometry: false,
        ui_show_image: false,
        ui_show_noise: false,
        size,
        vertex_buffer,
    }
}

/// Update app state.
/// WARNING: order is very important here.
/// The image uniforms use an update flag so other parts of the app know to update.
/// This flag is set by the interface and unset by the profram store.
/// Update order should be: interface, uniforms, shaders
fn update(app: &App, model: &mut app::Model, _update: Update) {
    if model.paused {
        return;
    }

    let window = app.window(model.main_window_id).unwrap();
    let device = window.swap_chain_device();

    if model.show_controls {
        interface::update(app, device, model);
    }

    model.program_store.update_uniforms(device);

    model
        .program_store
        .update_shaders(app, device, window.msaa_samples());
}

fn resize(app: &App, model: &mut app::Model, width: u32, height: u32) {
    let window = app.window(model.main_window_id).unwrap();
    window.set_inner_size_pixels(width, height);
}

fn pause(_app: &App, model: &mut app::Model) {
    model.paused = true;
    model.program_store.pause();
}

fn unpause(_app: &App, model: &mut app::Model) {
    model.paused = false;
    model.program_store.unpause();
}

/// Handle key pressed event
fn key_pressed(app: &App, model: &mut app::Model, key: Key) {
    match key {
        Key::H => model.show_controls = !model.show_controls,
        Key::Key1 => resize(app, model, 852, 480),
        Key::Key2 => resize(app, model, 1280, 720),
        Key::Key3 => resize(app, model, 1920, 1080),
        Key::Key4 => resize(app, model, 2560, 1440),
        Key::Key5 => resize(app, model, 3840, 2160),
        Key::Key0 => resize(app, model, model.original_width, model.original_height),
        Key::P => {
            if model.paused {
                unpause(app, model);
            } else {
                pause(app, model);
            }
        }
        Key::R => {
            model.program_store.buffer_store.general_uniforms.reset();
        }
        _ => (),
    };

    match &model.program_store.current_subscriptions {
        Some(current_subscriptions) => {
            if !current_subscriptions.camera {
                return;
            }
        }
        None => return,
    }

    let scale = 0.2;
    let theta = 0.01;

    let camera = &mut model.program_store.buffer_store.camera_uniforms;
    let camera_dir = camera.dir();
    let camera_up = camera.up();
    let cross = camera_dir.cross(camera_up);
    let cross_dir = util::normalize_vector(cross);

    match key {
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

fn resized(_app: &App, model: &mut app::Model, size: Vector2) {
    model.size = size;
    model
        .program_store
        .buffer_store
        .general_uniforms
        .set_size(size);
}

fn mouse_moved(_app: &App, model: &mut app::Model, position: Vector2) {
    model
        .program_store
        .buffer_store
        .general_uniforms
        .set_mouse(position);
}

fn mouse_pressed(_app: &App, model: &mut app::Model, _: nannou::event::MouseButton) {
    model
        .program_store
        .buffer_store
        .general_uniforms
        .data
        .mouse_down = 1;
}

fn mouse_released(_app: &App, model: &mut app::Model, _: nannou::event::MouseButton) {
    model
        .program_store
        .buffer_store
        .general_uniforms
        .data
        .mouse_down = 0;
}

/// Draw the state of the app to the frame
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
    let bind_groups = match model.program_store.get_bind_groups() {
        Some(g) => g,
        None => return false,
    };
    for (set, bind_group) in bind_groups.iter().enumerate() {
        render_pass.set_bind_group(set as u32, bind_group, &[]);
    }

    // render quad
    let vertex_range = 0..quad_2d::VERTICES.len() as u32;
    let instance_range = 0..1;
    render_pass.draw(vertex_range, instance_range);
    true
}

/// Render app
fn view(app: &App, model: &app::Model, frame: Frame) {
    if model.paused {
        thread::sleep(time::Duration::from_millis(500));
        return;
    }

    if !draw(model, &frame) {
        let draw = app.draw();
        draw.background().color(DARKGRAY);
        draw.to_frame(app, &frame).unwrap();
    }

    if model.show_controls {
        interface::draw(app, model, &frame);
    }
}
