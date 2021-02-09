use nannou::prelude::*;

/**
 * Generic interface
 */
pub trait Bufferable<T>: Sized {
    fn as_bytes(&self) -> &[u8];

    fn textures(&self) -> Option<Vec<&wgpu::Texture>> {
        None
    }
}
