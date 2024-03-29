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
    println!("num msaa samples: {:?}", msaa_samples);

    let desc = wgpu::CommandEncoderDescriptor {
        label: Some("nannou_isf_pipeline_new"),
    };
    let mut encoder = device.create_command_encoder(&desc);

    let (width, height) = window.inner_size_pixels();
    let size = pt2(width as f32, height as f32);
    let mut program_store = programs::ProgramStore::new(app, device, size, msaa_samples);
    program_store.configure(app, device, &mut encoder, msaa_samples, size);
    let vertex_buffer = quad_2d::create_vertex_buffer(device);

    let texture = util::create_app_texture(device, size, msaa_samples);
    let texture_reshaper = util::create_texture_reshaper(device, &texture, msaa_samples);

    window.swap_chain_queue().submit(vec![encoder.finish()]);

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
        texture,
        texture_reshaper,
        ui,
        ui_show_audio_fft: false,
        ui_show_color: false,
        ui_show_geometry: false,
        ui_show_image: false,
        ui_show_noise: false,
        ui_show_video: false,
        resized: false,
        size,
        vertex_buffer,
    }
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

fn resized(_app: &App, model: &mut app::Model, size: Vector2<f32>) {
    model.size = size;
    model
        .program_store
        .buffer_store
        .general_uniforms
        .set_size(size);
    model.resized = true;
}

fn mouse_moved(_app: &App, model: &mut app::Model, position: Vector2<f32>) {
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

fn update(app: &App, model: &mut app::Model, update: Update) {
    if model.paused {
        return;
    }

    let window = app.window(model.main_window_id).unwrap();
    let device = window.swap_chain_device();
    let num_samples = window.msaa_samples();

    model.encode_update(app, update, &window, device, num_samples);

    if model.program_store.is_multipass() {
        model.encode_render_passes(&window, device);
    }
}

/// Draw the state of the app to the frame
fn draw(model: &app::Model, frame: &Frame) {
    if model.program_store.is_multipass() {
        model.render_texture_to_frame(frame)
    } else {
        let device = frame.device_queue_pair().device();
        let mut encoder = frame.command_encoder();
        model.encode_render_pass(device, &mut *encoder, frame.texture_view());
    }
}

/// Render app
fn view(app: &App, model: &app::Model, frame: Frame) {
    if model.paused {
        thread::sleep(time::Duration::from_millis(500));
        return;
    }

    draw(model, &frame);

    interface::draw(app, model, &frame);
}
