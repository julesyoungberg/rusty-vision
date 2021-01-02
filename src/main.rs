use nannou::prelude::*;
use shaderc;
use std::fs;

static SIZE: u32 = 1024;

struct Model {
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

// The vertex type that we will use to represent a point on our triangle.
#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

// The vertices that make up the rectangle to which the image will be drawn.
const VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
];

fn main() {
    nannou::app(model).run();
}

fn compile_shader(
    device: &wgpu::Device,
    compiler: &mut shaderc::Compiler,
    filename: &str,
    kind: shaderc::ShaderKind,
) -> wgpu::ShaderModule {
    let src_string = fs::read_to_string(filename).expect("Error reading shader");
    let src = src_string.as_str();
    let spirv = compiler
        .compile_into_spirv(src, kind, filename, "main", None)
        .unwrap();
    return wgpu::shader_from_spirv_bytes(device, &spirv.as_binary_u8());
}

fn create_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
    let desc = wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[],
    };
    device.create_pipeline_layout(&desc)
}

fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    vs_mod: &wgpu::ShaderModule,
    fs_mod: &wgpu::ShaderModule,
    dst_format: wgpu::TextureFormat,
    sample_count: u32,
) -> wgpu::RenderPipeline {
    wgpu::RenderPipelineBuilder::from_layout(layout, vs_mod)
        .fragment_shader(fs_mod)
        .color_format(dst_format)
        .add_vertex_buffer::<Vertex>(&wgpu::vertex_attr_array![0 => Float2])
        .sample_count(sample_count)
        .primitive_topology(wgpu::PrimitiveTopology::TriangleStrip)
        .build(device)
}

// See the `nannou::wgpu::bytes` documentation for why this is necessary.
fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
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

    let mut compiler = shaderc::Compiler::new().unwrap();
    let vs_module = compile_shader(
        device,
        &mut compiler,
        "./src/shaders/basic.vert",
        shaderc::ShaderKind::Vertex,
    );
    let fs_module = compile_shader(
        device,
        &mut compiler,
        "./src/shaders/basic.frag",
        shaderc::ShaderKind::Fragment,
    );

    let pipeline_layout = create_pipeline_layout(device);
    let render_pipeline = create_render_pipeline(
        device,
        &pipeline_layout,
        &vs_module,
        &fs_module,
        Frame::TEXTURE_FORMAT,
        msaa_samples,
    );

    let vertices_bytes = vertices_as_bytes(&VERTICES[..]);
    let usage = wgpu::BufferUsage::VERTEX;
    let vertex_buffer = device.create_buffer_with_data(vertices_bytes, usage);

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
    let vertex_range = 0..VERTICES.len() as u32;
    let instance_range = 0..1;
    render_pass.draw(vertex_range, instance_range);
}
