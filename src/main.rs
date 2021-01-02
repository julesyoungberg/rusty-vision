use nannou::prelude::*;

mod d2;
mod shaders;
mod util;

static SIZE: u32 = 1024;

const SHADERS: &'static [&'static str] = &["basic.vert", "basic.frag"];

struct Model {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    let w_id = app
        .new_window()
        .size(SIZE, SIZE)
        .view(view)
        .build()
        .unwrap();
    let window = app.window(w_id).unwrap();
    let device = window.swap_chain_device();
    let msaa_samples = window.msaa_samples();

    let shaders = shaders::compile_shaders(device, SHADERS);
    let vert_shader = shaders::get_shader(&shaders, "basic.vert");
    let frag_shader = shaders::get_shader(&shaders, "basic.frag");
    let render_pipeline = util::create_pipeline(device, vert_shader, frag_shader, msaa_samples);
    let vertex_buffer = d2::create_vertex_buffer(device);

    Model {
        render_pipeline,
        vertex_buffer,
    }
}

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
