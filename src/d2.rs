use nannou::prelude::*;

#[path = "util.rs"]
mod util;

// The vertices that make up the rectangle to which the image will be drawn.
pub const VERTICES: [util::Vertex; 4] = [
    util::Vertex {
        position: [-1.0, 1.0],
    },
    util::Vertex {
        position: [-1.0, -1.0],
    },
    util::Vertex {
        position: [1.0, 1.0],
    },
    util::Vertex {
        position: [1.0, -1.0],
    },
];

pub fn create_vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    let vertices_bytes = util::vertices_as_bytes(&VERTICES[..]);
    let usage = wgpu::BufferUsage::VERTEX;
    let vertex_buffer = device.create_buffer_with_data(vertices_bytes, usage);
    return vertex_buffer;
}
