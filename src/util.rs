#![allow(dead_code)]

use nannou::math::cgmath::Matrix3;
use nannou::prelude::*;

// The vertex type that we will use to represent a point on our triangle.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 2],
}

fn create_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    let desc = wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
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

pub fn create_pipeline(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
    vs: &wgpu::ShaderModule,
    fs: &wgpu::ShaderModule,
    sample_count: u32,
) -> wgpu::RenderPipeline {
    let pipeline_layout = create_pipeline_layout(device, bind_group_layout);
    create_render_pipeline(
        device,
        &pipeline_layout,
        vs,
        fs,
        Frame::TEXTURE_FORMAT,
        sample_count,
    )
}

// See the `nannou::wgpu::bytes` documentation for why this is necessary.
pub fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}

pub fn rotate_around_axis(axis: Vector3, theta: f32) -> Matrix3<f32> {
    let cos = theta.cos();
    let sin = theta.sin();
    let m00 = cos + axis.x * axis.x * (1.0 - cos);
    let m10 = axis.x * axis.y * (1.0 - cos) - axis.z * sin;
    let m20 = axis.x * axis.z * (1.0 - cos) + axis.y * sin;
    let m01 = axis.y * axis.x * (1.0 - cos) + axis.z * sin;
    let m11 = cos + axis.y * axis.y * (1.0 - cos);
    let m21 = axis.y * axis.z * (1.0 - cos) - axis.x * sin;
    let m02 = axis.z * axis.x * (1.0 - cos) - axis.y * sin;
    let m12 = axis.z * axis.y * (1.0 - cos) + axis.x * sin;
    let m22 = cos + axis.z * axis.z * (1.0 - cos);
    Matrix3::new(m00, m01, m02, m10, m11, m12, m20, m21, m22)
}
