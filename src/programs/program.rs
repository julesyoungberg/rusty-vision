use nannou::prelude::*;
use rayon::prelude::*;
use std::collections::HashMap;

use crate::programs::config;
use crate::programs::shaders;
use crate::util;

pub type ProgramErrors = HashMap<String, String>;

/// represents a GPU program (series of shaders).
/// Manages the compilation of code and
/// creation of the program as a GPU Render Pipeline.
#[derive(Debug)]
pub struct Program {
    pub config: config::ProgramConfig,
    pub errors: ProgramErrors,
    pub pipeline: Option<wgpu::RenderPipeline>,

    frag_shader: shaders::Shader,
    vert_shader: shaders::Shader,
}

impl Program {
    pub fn new(config: config::ProgramConfig, folder_name: String) -> Self {
        let frag_name = format!("{}/{}", folder_name, config.pipeline.frag);
        let mut vert_name = "default.vert".to_owned();
        if let Some(name) = &config.pipeline.vert {
            vert_name = format!("{}/{}", folder_name, name);
        }

        Self {
            config,
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

    /// Compile the program with the latest shader code.
    pub fn compile(&mut self, app: &App, device: &wgpu::Device) {
        let mut shaders = [&mut self.vert_shader, &mut self.frag_shader];
        let path = util::shaders_path(app);

        // compile shaders
        shaders.par_iter_mut().for_each(|shader| {
            let mut compiler = shaderc::Compiler::new().unwrap();
            shader.compile(path.clone(), device, &mut compiler);
        });

        // collect errors
        self.errors = shaders.iter().fold(HashMap::new(), |mut errors, shader| {
            if let Some(e) = &shader.error {
                errors.insert(shader.filename.to_string(), e.to_string());
            }
            errors
        });
    }

    /// Create the render pipeline
    pub fn create_render_pipeline(
        &mut self,
        device: &wgpu::Device,
        layout_desc: &wgpu::PipelineLayoutDescriptor,
        num_samples: u32,
    ) {
        if self.errors.keys().len() > 0 {
            self.pipeline = None;
            return;
        }

        if let Some(vert_module) = &self.vert_shader.module {
            if let Some(frag_module) = &self.frag_shader.module {
                println!("creating pipeline");
                self.pipeline = Some(util::create_pipeline(
                    device,
                    layout_desc,
                    vert_module,
                    frag_module,
                    num_samples,
                ));
            }
        }
    }
}
