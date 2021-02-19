use nannou::prelude::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {}

/// Generic interface
pub trait Bufferable<T = Data>: Sized {
    fn as_bytes(&self) -> &[u8] {
        &[]
    }

    fn textures(&self) -> Vec<&wgpu::Texture> {
        vec![]
    }
}
