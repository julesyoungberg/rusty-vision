use nannou::prelude::*;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::{channel, Receiver};
use std::time;

mod d2;
mod pipelines;
mod shaders;
mod util;

static SIZE: u32 = 1024;

const SHADERS: &'static [&'static str] = &["basic.vert", "basic.frag"];

const PIPELINES: &'static [&'static [&'static str]] = &[&["basic", "basic.vert", "basic.frag"]];

#[allow(dead_code)]
struct Model {
    main_window_id: WindowId,
    render_pipeline: wgpu::RenderPipeline,
    shader_channel: Receiver<DebouncedEvent>,
    shader_watcher: notify::FsEventWatcher,
    vertex_buffer: wgpu::Buffer,
}

fn main() {
    nannou::app(model).update(update).run();
}

/**
 * App setup
 */
fn model(app: &App) -> Model {
    // create window
    let main_window_id = app
        .new_window()
        .size(SIZE, SIZE)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(main_window_id).unwrap();
    let device = window.swap_chain_device();
    let msaa_samples = window.msaa_samples();

    // setup shader watcher
    let (schannel, shader_channel) = channel();
    let mut shader_watcher = watcher(schannel, time::Duration::from_secs(1)).unwrap();
    shader_watcher
        .watch(shaders::SHADERS_PATH, RecursiveMode::Recursive)
        .unwrap();

    // compile shaders, build pipelines, and create GPU buffers
    let shaders = shaders::compile_shaders(device, SHADERS);
    let mut pipelines = pipelines::create_pipelines(device, msaa_samples, &shaders, &PIPELINES);
    let render_pipeline = pipelines.remove(&"basic").expect("Pipeline not found");
    let vertex_buffer = d2::create_vertex_buffer(device);

    Model {
        main_window_id,
        render_pipeline,
        shader_channel,
        shader_watcher,
        vertex_buffer,
    }
}

/**
 * Update app state
 */
fn update(app: &App, model: &mut Model, _update: Update) {
    // check for shader changes
    if let Ok(event) = model
        .shader_channel
        .recv_timeout(time::Duration::from_millis(10))
    {
        if let DebouncedEvent::Write(path) = event {
            let path_str = path.into_os_string().into_string().unwrap();
            println!("changes written to: {}", path_str);
            // changes have been made, recompile the shaders and rebuild the pipelines
            let window = app.window(model.main_window_id).unwrap();
            let device = window.swap_chain_device();
            let shaders = shaders::compile_shaders(device, SHADERS);
            let mut pipelines =
                pipelines::create_pipelines(device, window.msaa_samples(), &shaders, &PIPELINES);
            model.render_pipeline = pipelines.remove(&"basic").expect("Pipeline not found");
        }
    }
}

/**
 * Render app
 */
fn view(_app: &App, model: &Model, frame: Frame) {
    let mut encoder = frame.command_encoder();
    let mut render_pass = wgpu::RenderPassBuilder::new()
        .color_attachment(frame.texture_view(), |color| color)
        .begin(&mut encoder);

    render_pass.set_pipeline(&model.render_pipeline);
    render_pass.set_vertex_buffer(0, &model.vertex_buffer, 0, 0);

    let vertex_range = 0..d2::VERTICES.len() as u32;
    let instance_range = 0..1;

    render_pass.draw(vertex_range, instance_range);
}
