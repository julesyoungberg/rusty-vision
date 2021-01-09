#![allow(dead_code)]
use nannou::math::cgmath::Matrix4;
use nannou::prelude::*;

// The vertex type that we will use to represent a point on our triangle.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pub position: [f32; 2],
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
    layout_desc: &wgpu::PipelineLayoutDescriptor,
    vs: &wgpu::ShaderModule,
    fs: &wgpu::ShaderModule,
    sample_count: u32,
) -> wgpu::RenderPipeline {
    let pipeline_layout = device.create_pipeline_layout(layout_desc);
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

pub fn rotate_around_axis(axis: Vector3, theta: f32) -> Matrix4<f32> {
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
    Matrix4::new(
        m00, m01, m02, 0.0, m10, m11, m12, 0.0, m20, m21, m22, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}

pub fn transform_vector(transform: &Matrix4<f32>, vector: Vector3) -> Vector3 {
    let point = Point3::new(vector.x, vector.y, vector.z);
    let transformed_point = Transform::transform_point(transform, point.into());
    Vector3::new(
        transformed_point.x,
        transformed_point.y,
        transformed_point.z,
    )
}

pub fn vector_length(vector: Vector3) -> f32 {
    let sum = vector.x * vector.x + vector.y * vector.y + vector.z * vector.z;
    sum.sqrt()
}

#[cfg(test)]
#[test]
fn test_vector_length() {
    assert_eq!(vector_length(pt3(1.0, 2.0, 3.0)), 3.7416575);
}

pub fn normalize_vector(vector: Vector3) -> Vector3 {
    let len = vector_length(vector);
    pt3(vector.x / len, vector.y / len, vector.z / len)
}

#[cfg(test)]
#[test]
fn test_normalize_vector() {
    let normalized = normalize_vector(pt3(1.0, 2.0, 3.0));
    assert_eq!(normalized.x, 0.26726124);
    assert_eq!(normalized.y, 0.5345225);
    assert_eq!(normalized.z, 0.8017837);
}
