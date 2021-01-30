use nannou::prelude::*;
use rayon::prelude::*;
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
    pub fn new(vert_name: String, frag_name: String) -> Self {
        Self {
            errors: HashMap::new(),
            frag_shader: shaders::Shader::new(frag_name),
            pipeline: None,
            vert_shader: shaders::Shader::new(vert_name),
        }
    }

    pub fn is_new(&self) -> bool {
        self.pipeline.is_none() && self.errors.keys().len() == 0
    }

    pub fn clear(&mut self) {
        self.errors = HashMap::new();
        self.pipeline = None;
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
        let mut shaders = [&mut self.vert_shader, &mut self.frag_shader];

        // compile shaders
        shaders.par_iter_mut().for_each(|shader| {
            let mut compiler = shaderc::Compiler::new().unwrap();
            shader.compile(device, &mut compiler);
        });

        // collect errors
        self.errors = shaders.iter().fold(HashMap::new(), |mut errors, shader| {
            if let Some(e) = &shader.error {
                errors.insert(shader.filename.to_string(), e.to_string());
            }
            errors
        });

        // exit early if errors
        if self.errors.keys().len() > 0 {
            self.pipeline = None;
            return;
        }

        // collect modules
        let modules = shaders
            .iter()
            .map(|shader| shader.module.as_ref().unwrap())
            .collect::<Vec<&wgpu::ShaderModule>>();

        // create the render pipeline and clear errors
        let pipeline =
            util::create_pipeline(device, layout_desc, modules[0], modules[1], num_samples);
        self.pipeline = Some(pipeline);
        self.errors = HashMap::new();
    }
}
