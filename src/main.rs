use nannou::prelude::*;
use nannou::ui::DrawToFrameError;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time;

mod app;
mod config;
mod d2;
mod interface;
mod pipelines;
mod shaders;
mod uniforms;
mod util;

fn main() {
    nannou::app(model).update(update).run();
}

fn create_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    wgpu::BindGroupLayoutBuilder::new()
        .uniform_buffer(wgpu::ShaderStage::FRAGMENT, false)
        .build(device)
}

fn create_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    uniform_buffer: &wgpu::Buffer,
) -> wgpu::BindGroup {
    wgpu::BindGroupBuilder::new()
        .buffer::<uniforms::Data>(uniform_buffer, 0..1)
        .build(device, layout)
}

/**
 * App setup
 */
fn model(app: &App) -> app::Model {
    // create window
    let main_window_id = app
        .new_window()
        .size(config::SIZE, config::SIZE)
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(main_window_id).unwrap();
    let device = window.swap_chain_device();
    let msaa_samples = window.msaa_samples();

    // setup uniform buffer
    let uniform = uniforms::Uniforms::new(pt2(config::SIZE as f32, config::SIZE as f32));
    let usage = wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST;
    let uniform_buffer = device.create_buffer_with_data(uniform.as_bytes(), usage);

    // create bind group
    let bind_group_layout = create_bind_group_layout(device);
    let bind_group = create_bind_group(device, &bind_group_layout, &uniform_buffer);

    // setup shader watcher
    let (schannel, shader_channel) = channel();
    let mut shader_watcher = watcher(schannel, time::Duration::from_secs(1)).unwrap();
    shader_watcher
        .watch(config::SHADERS_PATH, RecursiveMode::Recursive)
        .unwrap();

    // compile shaders, build pipelines, and create GPU buffers
    let shaders = shaders::compile_shaders(device, config::SHADERS);
    let pipelines = pipelines::create_pipelines(
        device,
        &bind_group_layout,
        msaa_samples,
        &shaders,
        &config::PIPELINES,
    );
    let vertex_buffer = d2::create_vertex_buffer(device);

    // create UI
    let mut ui = app.new_ui().build().unwrap();
    let widget_ids = app::WidgetIds::new(ui.widget_id_generator());

    app::Model {
        bind_group,
        bind_group_layout,
        current_program: 2,
        widget_ids,
        main_window_id,
        pipelines,
        shader_channel,
        shader_watcher,
        show_controls: true,
        ui,
        ui_show_general: false,
        uniforms: uniform,
        uniform_buffer,
        vertex_buffer,
    }
}

fn update_shaders(app: &App, model: &mut app::Model) {
    // check for shader changes
    if let Ok(event) = model
        .shader_channel
        .recv_timeout(time::Duration::from_millis(10))
    {
        // received event from notify - check if a write has been made
        if let DebouncedEvent::Write(path) = event {
            let path_str = path.into_os_string().into_string().unwrap();
            println!("changes written to: {}", path_str);
            // changes have been made, recompile the shaders and rebuild the pipelines
            let window = app.window(model.main_window_id).unwrap();
            let device = window.swap_chain_device();
            let shaders = shaders::compile_shaders(device, config::SHADERS);
            model.pipelines = pipelines::create_pipelines(
                device,
                &model.bind_group_layout,
                window.msaa_samples(),
                &shaders,
                &config::PIPELINES,
            );
        }
    }
}

/**
 * Update app state
 */
fn update(app: &App, model: &mut app::Model, _update: Update) {
    model.uniforms.update_time();
    update_shaders(app, model);
    interface::update_ui(model);
}

/**
 * Handle key pressed event
 */
fn key_pressed(_app: &App, model: &mut app::Model, key: Key) {
    let camera_dir = model.uniforms.camera_dir();
    let scale = 0.2;

    let camera_up = model.uniforms.camera_up();
    let cross = camera_dir.cross(camera_up);
    let sum = cross.x * cross.x + cross.y * cross.y + cross.z * cross.z;
    let len = sum.sqrt();
    let cross_dir = pt3(cross.x / len, cross.y / len, cross.z / len);

    if key == Key::H {
        model.show_controls = !model.show_controls;
    } else if key == Key::Up {
        model.uniforms.data.camera_pos_x = model.uniforms.data.camera_pos_x + camera_dir.x * scale;
        model.uniforms.data.camera_pos_y = model.uniforms.data.camera_pos_y + camera_dir.y * scale;
        model.uniforms.data.camera_pos_z = model.uniforms.data.camera_pos_z + camera_dir.z * scale;
        model.uniforms.data.camera_target_x =
            model.uniforms.data.camera_target_x + camera_dir.x * scale;
        model.uniforms.data.camera_target_y =
            model.uniforms.data.camera_target_y + camera_dir.y * scale;
        model.uniforms.data.camera_target_z =
            model.uniforms.data.camera_target_z + camera_dir.z * scale;
    } else if key == Key::Down {
        model.uniforms.data.camera_pos_x = model.uniforms.data.camera_pos_x - camera_dir.x * scale;
        model.uniforms.data.camera_pos_y = model.uniforms.data.camera_pos_y - camera_dir.y * scale;
        model.uniforms.data.camera_pos_z = model.uniforms.data.camera_pos_z - camera_dir.z * scale;
        model.uniforms.data.camera_target_x =
            model.uniforms.data.camera_target_x - camera_dir.x * scale;
        model.uniforms.data.camera_target_y =
            model.uniforms.data.camera_target_y - camera_dir.y * scale;
        model.uniforms.data.camera_target_z =
            model.uniforms.data.camera_target_z - camera_dir.z * scale;
    } else if key == Key::Left {
        model.uniforms.data.camera_pos_x = model.uniforms.data.camera_pos_x - cross_dir.x * scale;
        model.uniforms.data.camera_pos_y = model.uniforms.data.camera_pos_y - cross_dir.y * scale;
        model.uniforms.data.camera_pos_z = model.uniforms.data.camera_pos_z - cross_dir.z * scale;
        model.uniforms.data.camera_target_x =
            model.uniforms.data.camera_target_x - cross_dir.x * scale;
        model.uniforms.data.camera_target_y =
            model.uniforms.data.camera_target_y - cross_dir.y * scale;
        model.uniforms.data.camera_target_z =
            model.uniforms.data.camera_target_z - cross_dir.z * scale;
    } else if key == Key::Right {
        model.uniforms.data.camera_pos_x = model.uniforms.data.camera_pos_x + cross_dir.x * scale;
        model.uniforms.data.camera_pos_y = model.uniforms.data.camera_pos_y + cross_dir.y * scale;
        model.uniforms.data.camera_pos_z = model.uniforms.data.camera_pos_z + cross_dir.z * scale;
        model.uniforms.data.camera_target_x =
            model.uniforms.data.camera_target_x + cross_dir.x * scale;
        model.uniforms.data.camera_target_y =
            model.uniforms.data.camera_target_y + cross_dir.y * scale;
        model.uniforms.data.camera_target_z =
            model.uniforms.data.camera_target_z + cross_dir.z * scale;
    } // else if key == Key::W {
      //     // rotate camera dir and camera up around cross_dir
      //     let theta = 0.2;
      //     let rotation_matrix = util::rotate_around_axis(cross_dir, theta);
      //     let next_dir = rotation_matrix.transform_point(camera_dir.into());
      // } else if key == Key::S {
      //     // rotate camera dir and camera up around cross_dir
      // }
}

/**
 * Draw the state of the app to the frame
 */
fn draw(model: &app::Model, frame: &Frame) {
    // setup environment
    let device = frame.device_queue_pair().device();
    let render_pipeline = model
        .pipelines
        .get(config::PROGRAMS[model.current_program])
        .expect("Invalid program");
    let mut encoder = frame.command_encoder();

    // update uniform buffer
    let uniforms_size = std::mem::size_of::<uniforms::Data>() as wgpu::BufferAddress;
    let uniforms_bytes = model.uniforms.as_bytes();
    let uniforms_usage = wgpu::BufferUsage::COPY_SRC;
    let new_uniform_buffer = device.create_buffer_with_data(uniforms_bytes, uniforms_usage);
    encoder.copy_buffer_to_buffer(
        &new_uniform_buffer,
        0,
        &model.uniform_buffer,
        0,
        uniforms_size,
    );

    // configure pipeline
    let mut render_pass = wgpu::RenderPassBuilder::new()
        .color_attachment(&frame.texture_view(), |color| color)
        .begin(&mut encoder);
    render_pass.set_pipeline(&render_pipeline);
    render_pass.set_vertex_buffer(0, &model.vertex_buffer, 0, 0);
    render_pass.set_bind_group(0, &model.bind_group, &[]);

    // render
    let vertex_range = 0..d2::VERTICES.len() as u32;
    let instance_range = 0..1;
    render_pass.draw(vertex_range, instance_range);
}

/**
 * Draw the state of the `Ui` to the frame.
 */
fn draw_ui(app: &App, model: &app::Model, frame: &Frame) {
    // let draw = app.draw();
    // draw.quad().color(STEELBLUE).x_y(0.0, 0.0).w_h(200.0, 200.0);
    // draw.to_frame(app, &frame).unwrap();

    let color_attachment_desc = frame.color_attachment_descriptor();
    let primitives = model.ui.draw();
    let window = app
        .window(model.main_window_id)
        .ok_or(DrawToFrameError::InvalidWindow)
        .unwrap();
    let mut ui_encoder = frame.command_encoder();
    ui::encode_render_pass(
        &model.ui,
        &window,
        primitives,
        color_attachment_desc,
        &mut *ui_encoder,
    )
    .unwrap();
}

/**
 * Render app
 */
fn view(app: &App, model: &app::Model, frame: Frame) {
    draw(model, &frame);
    if model.show_controls {
        draw_ui(app, model, &frame);
    }
}
