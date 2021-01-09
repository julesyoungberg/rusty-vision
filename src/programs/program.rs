use nannou::prelude::*;
use shaderc;
use std::collections::HashMap;

use crate::programs::shaders;
use crate::util;

pub type ProgramErrors = HashMap<String, String>;

/**
 * represents a GPU program (series of shaders)
 */
#[derive(Debug)]
pub struct Program {
    pub errors: ProgramErrors,
    pub frag_shader: shaders::Shader,
    pub pipeline: Option<wgpu::RenderPipeline>,
    pub vert_shader: shaders::Shader,
}

/**
 * Manages the compilation of code and
 * creation of the program as a GPU Render Pipeline
 */
impl Program {
    pub fn new(vert_name: &str, frag_name: &str) -> Self {
        Self {
            errors: HashMap::new(),
            frag_shader: shaders::Shader::new(frag_name.to_string()),
            pipeline: None,
            vert_shader: shaders::Shader::new(vert_name.to_string()),
        }
    }

    /**
     * Compile the program with the latest shader code.
     */
    pub fn compile(
        &mut self,
        device: &wgpu::Device,
        layout_desc: &wgpu::PipelineLayoutDescriptor,
        num_samples: u32,
    ) {
        let mut compiler = shaderc::Compiler::new().unwrap();

        // compile shaders
        let mut shaders = [&mut self.vert_shader, &mut self.frag_shader];
        shaders.iter_mut().for_each(|shader| {
            shader.compile(device, &mut compiler);
        });

        // collect errors
        self.errors = shaders.iter().fold(HashMap::new(), |mut errors, shader| {
            if let Some(e) = &shader.error {
                errors.insert(shader.filename.to_string(), e.to_string());
            }
            errors
        });

        // exit early if any errors
        if self.errors.keys().len() > 0 {
            self.pipeline = None;
            return;
        }

        let vert_shader = self.vert_shader.module.as_ref().unwrap();
        let frag_shader = self.frag_shader.module.as_ref().unwrap();

        // both shaders are valid, create the render pipeline and clear the error
        let pipeline =
            util::create_pipeline(device, layout_desc, vert_shader, frag_shader, num_samples);
        self.pipeline = Some(pipeline);
        self.errors = HashMap::new();
    }
}
