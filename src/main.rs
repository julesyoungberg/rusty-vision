use nannou::prelude::*;
use nannou::ui::prelude::*;
use nannou::ui::DrawToFrameError;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::{channel, Receiver};
use std::time;

mod d2;
mod pipelines;
mod shaders;
mod util;

static SIZE: u32 = 1024;

const SHADERS: &'static [&'static str] = &["basic.vert", "basic.frag", "basic2.frag"];

const PIPELINES: &'static [&'static [&'static str]] = &[
    &["basic", "basic.vert", "basic.frag"],
    &["basic2", "basic.vert", "basic2.frag"],
];

const PROGRAMS: &'static [&'static str] = &["basic", "basic2"];

#[allow(dead_code)]
struct Model {
    current_program: usize,
    ids: Ids,
    main_window_id: WindowId,
    render_pipeline: wgpu::RenderPipeline,
    shader_channel: Receiver<DebouncedEvent>,
    shader_watcher: notify::FsEventWatcher,
    ui: Ui,
    vertex_buffer: wgpu::Buffer,
}

widget_ids! {
    struct Ids {
        current_program,
    }
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

    // create UI
    let mut ui = app.new_ui().build().unwrap();

    // generate ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    Model {
        current_program: 0,
        ids,
        main_window_id,
        render_pipeline,
        shader_channel,
        shader_watcher,
        ui,
        vertex_buffer,
    }
}

fn update_shaders(app: &App, model: &mut Model) {
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
            let shaders = shaders::compile_shaders(device, SHADERS);
            let mut pipelines =
                pipelines::create_pipelines(device, window.msaa_samples(), &shaders, &PIPELINES);
            model.render_pipeline = pipelines.remove(&"basic").expect("Pipeline not found");
        }
    }
}

fn update_ui(model: &mut Model) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();

    for selected in widget::DropDownList::new(PROGRAMS, Option::from(model.current_program))
        .w_h(200.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .border(0.0)
        .top_left_with_margin(20.0)
        .label("Current Program")
        .set(model.ids.current_program, ui)
    {
        if selected != model.current_program {
            println!("program selected: {}", PROGRAMS[selected]);
            model.current_program = selected;
        }
    }
}

/**
 * Update app state
 */
fn update(app: &App, model: &mut Model, _update: Update) {
    update_shaders(app, model);
    update_ui(model);
}

/**
 * Draw the state of the app to the frame
 */
fn draw(model: &Model, frame: &Frame) {
    let mut encoder = frame.command_encoder();
    let mut render_pass = wgpu::RenderPassBuilder::new()
        .color_attachment(&frame.texture_view(), |color| color)
        .begin(&mut encoder);

    render_pass.set_pipeline(&model.render_pipeline);
    render_pass.set_vertex_buffer(0, &model.vertex_buffer, 0, 0);

    let vertex_range = 0..d2::VERTICES.len() as u32;
    let instance_range = 0..1;

    render_pass.draw(vertex_range, instance_range);
}

/**
 * Draw the state of the `Ui` to the frame.
 */
fn draw_ui(app: &App, model: &Model, frame: &Frame) {
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
fn view(app: &App, model: &Model, frame: Frame) {
    draw(model, &frame);
    draw_ui(app, model, &frame);
}
