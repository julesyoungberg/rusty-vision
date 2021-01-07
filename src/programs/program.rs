use nannou::prelude::*;

use crate::programs::shaders;
use crate::util;

/**
 * represents a GPU program (series of shaders)
 */
pub struct Program {
    pub error: Option<String>,
    pub pipeline: Option<wgpu::RenderPipeline>,
    frag_name: String,
    vert_name: String,
}

/**
 * Manages the creation of the program as a GPU Render Pipeline
 */
impl Program {
    pub fn new(vert_name: &str, frag_name: &str) -> Self {
        Self {
            error: None,
            frag_name: frag_name.to_string(),
            pipeline: None,
            vert_name: vert_name.to_string(),
        }
    }

    /**
     * Update the program with the latest shaders and handle
     * any compilation errors.
     */
    pub fn update(
        &mut self,
        device: &wgpu::Device,
        bind_group_layout: &wgpu::BindGroupLayout,
        num_samples: u32,
        shaders: &shaders::Shaders,
    ) {
        // fetch the vertex shader, exit early if theres errors
        let vert = shaders.get(&self.vert_name).unwrap();
        if let Some(e) = &vert.error {
            self.error = Some(e.to_string());
            self.pipeline = None;
            return;
        }
        let vert_shader = vert.module.as_ref().unwrap();

        // fetch the fragment shader, exit early if theres errors
        let frag = shaders.get(&self.frag_name).unwrap();
        if let Some(e) = &frag.error {
            self.error = Some(e.to_string());
            self.pipeline = None;
            return;
        }
        let frag_shader = frag.module.as_ref().unwrap();

        // both shaders are valid, create the render pipeline
        let pipeline = util::create_pipeline(
            device,
            bind_group_layout,
            vert_shader,
            frag_shader,
            num_samples,
        );
        self.pipeline = Some(pipeline);
        self.error = None;
    }
}
