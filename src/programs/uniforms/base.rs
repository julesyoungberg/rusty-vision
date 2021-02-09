use nannou::prelude::*;

use crate::programs::config;

/**
 * Generic interface
 */
pub trait Bufferable: Sized {
    fn as_bytes(&self) -> &[u8];

    fn textures(&self) -> Option<Vec<&wgpu::Texture>> {
        None
    }

    fn set_program_defaults(&mut self, _defaults: &Option<config::ProgramDefaults>) {}
}
