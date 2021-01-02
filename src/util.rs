use nannou::prelude::*;

// The vertex type that we will use to represent a point on our triangle.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 2],
}

pub fn create_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
    let desc = wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[],
    };
    device.create_pipeline_layout(&desc)
}

pub fn create_render_pipeline(
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
pub fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}
