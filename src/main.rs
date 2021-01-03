use nannou::prelude::*;
use nannou::ui::prelude::*;
use nannou::ui::DrawToFrameError;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::{channel, Receiver};
use std::time;

mod d2;
mod pipelines;
mod shaders;
mod uniforms;
mod util;

static SIZE: u32 = 1024;

const SHADERS: &'static [&'static str] =
    &["basic.vert", "basic.frag", "basic2.frag", "mandelbox.frag"];

const PIPELINES: &'static [&'static [&'static str]] = &[
    &["basic", "basic.vert", "basic.frag"],
    &["basic2", "basic.vert", "basic2.frag"],
    &["mandelbox", "basic.vert", "mandelbox.frag"],
];

const PROGRAMS: &'static [&'static str] = &["basic", "basic2", "mandelbox"];

const COLOR_MODES: &'static [&'static str] = &["palette", "solid"];

#[allow(dead_code)]
struct Model {
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    current_program: usize,
    ids: Ids,
    main_window_id: WindowId,
    pipelines: pipelines::Pipelines,
    shader_channel: Receiver<DebouncedEvent>,
    shader_watcher: notify::FsEventWatcher,
    ui: Ui,
    uniforms: uniforms::Uniforms,
    uniform_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
}

widget_ids! {
    struct Ids {
        color_mode,
        current_program,
        draw_floor,
        fog_dist,
        quality,
    }
}

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

    // setup uniform buffer
    let uniform = uniforms::Uniforms::new(pt2(SIZE as f32, SIZE as f32));
    let usage = wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST;
    let uniform_buffer = device.create_buffer_with_data(uniform.as_bytes(), usage);

    // create bind group
    let bind_group_layout = create_bind_group_layout(device);
    let bind_group = create_bind_group(device, &bind_group_layout, &uniform_buffer);

    // setup shader watcher
    let (schannel, shader_channel) = channel();
    let mut shader_watcher = watcher(schannel, time::Duration::from_secs(1)).unwrap();
    shader_watcher
        .watch(shaders::SHADERS_PATH, RecursiveMode::Recursive)
        .unwrap();

    // compile shaders, build pipelines, and create GPU buffers
    let shaders = shaders::compile_shaders(device, SHADERS);
    let pipelines = pipelines::create_pipelines(
        device,
        &bind_group_layout,
        msaa_samples,
        &shaders,
        &PIPELINES,
    );
    let vertex_buffer = d2::create_vertex_buffer(device);

    // create UI
    let mut ui = app.new_ui().build().unwrap();
    let ids = Ids::new(ui.widget_id_generator());

    Model {
        bind_group,
        bind_group_layout,
        current_program: 2,
        ids,
        main_window_id,
        pipelines,
        shader_channel,
        shader_watcher,
        ui,
        uniforms: uniform,
        uniform_buffer,
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
            model.pipelines = pipelines::create_pipelines(
                device,
                &model.bind_group_layout,
                window.msaa_samples(),
                &shaders,
                &PIPELINES,
            );
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

    for selected in widget::DropDownList::new(
        COLOR_MODES,
        Option::from(model.uniforms.data.color_mode as usize),
    )
    .w_h(200.0, 30.0)
    .label_font_size(15)
    .rgb(0.3, 0.3, 0.3)
    .label_rgb(1.0, 1.0, 1.0)
    .border(0.0)
    .down(10.0)
    .label("Color Mode")
    .set(model.ids.color_mode, ui)
    {
        if selected as i32 != model.uniforms.data.color_mode {
            println!("color mode selected: {}", COLOR_MODES[selected]);
            model.uniforms.data.color_mode = selected as i32;
        }
    }

    let mut floor_btn_color = 0.3;
    let mut floor_btn_label = 1.0;
    if model.uniforms.data.draw_floor {
        floor_btn_color = 0.7;
        floor_btn_label = 0.0;
    }

    for _click in widget::Button::new()
        .down(10.0)
        .w_h(200.0, 30.0)
        .label_font_size(15)
        .label("Draw Floor")
        .rgb(floor_btn_color, floor_btn_color, floor_btn_color)
        .label_rgb(floor_btn_label, floor_btn_label, floor_btn_label)
        .border(0.0)
        .set(model.ids.draw_floor, ui)
    {
        model.uniforms.data.draw_floor = !model.uniforms.data.draw_floor;
    }

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    for value in slider(model.uniforms.data.fog_dist, 15.0, 300.0)
        .down(10.0)
        .label("Fog Distance")
        .set(model.ids.fog_dist, ui)
    {
        model.uniforms.data.fog_dist = value;
    }

    for value in slider(model.uniforms.data.quality, 1.0, 3.0)
        .down(10.0)
        .label("Quality")
        .set(model.ids.quality, ui)
    {
        model.uniforms.data.quality = value;
    }
}

/**
 * Update app state
 */
fn update(app: &App, model: &mut Model, _update: Update) {
    model.uniforms.update_time();
    update_shaders(app, model);
    update_ui(model);
}

/**
 * Draw the state of the app to the frame
 */
fn draw(model: &Model, frame: &Frame) {
    // setup environment
    let device = frame.device_queue_pair().device();
    let render_pipeline = model
        .pipelines
        .get(PROGRAMS[model.current_program])
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
