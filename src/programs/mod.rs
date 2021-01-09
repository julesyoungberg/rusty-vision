use nannou::prelude::*;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver};
use std::time;

use crate::config;

pub mod geometry_uniforms;
mod program;
mod shaders;
mod uniform_buffer;
pub mod uniforms;

use crate::programs::uniforms::Bufferable;

pub type Programs = HashMap<String, program::Program>;

/**
 * Stores GPU programs and related data
 */
pub struct ProgramStore {
    pub changes_channel: Receiver<DebouncedEvent>,
    pub current_program: usize,
    pub geometry_uniforms: geometry_uniforms::GeometryUniforms,
    pub geometry_uniform_buffer: uniform_buffer::UniformBuffer,
    pub programs: Programs,
    pub shader_store: shaders::ShaderStore,
    pub shader_watcher: notify::FsEventWatcher,
    pub uniforms: uniforms::Uniforms,
    pub uniform_buffer: uniform_buffer::UniformBuffer,
}

/**
 * Manages the maintenance of shader programs.
 * - listens to directory
 * - compiles code
 * - manages modules
 * - handles errors
 * - builds render pipelines
 * - manages uniform buffers
 */
impl ProgramStore {
    pub fn new(device: &wgpu::Device) -> Self {
        // create uniform buffer
        let mut uniforms =
            uniforms::Uniforms::new(pt2(config::SIZE[0] as f32, config::SIZE[1] as f32));
        uniforms.set_program_defaults(config::DEFAULT_PROGRAM);
        let uniform_buffer =
            uniform_buffer::UniformBuffer::new::<uniforms::Data>(device, uniforms.as_bytes());

        // setup geometry uniform buffer
        let mut geometry_uniforms = geometry_uniforms::GeometryUniforms::new();
        geometry_uniforms.set_program_defaults(config::DEFAULT_PROGRAM);
        let geometry_uniform_buffer = uniform_buffer::UniformBuffer::new::<geometry_uniforms::Data>(
            device,
            geometry_uniforms.as_bytes(),
        );

        // setup shader watcher
        let (send_channel, changes_channel) = channel();
        let mut shader_watcher = watcher(send_channel, time::Duration::from_secs(1)).unwrap();
        shader_watcher
            .watch(config::SHADERS_PATH, RecursiveMode::Recursive)
            .unwrap();

        let mut programs = HashMap::new();
        for pipeline_desc in config::PIPELINES {
            let name = String::from(pipeline_desc[0]);
            programs.insert(
                name,
                program::Program::new(pipeline_desc[1], pipeline_desc[2]),
            );
        }

        Self {
            changes_channel,
            current_program: config::DEFAULT_PROGRAM,
            geometry_uniforms,
            geometry_uniform_buffer,
            programs,
            shader_store: shaders::ShaderStore::new(),
            shader_watcher,
            uniforms,
            uniform_buffer,
        }
    }

    /**
     * Compile all shaders and [re]create pipelines.
     * Call once after initialization.
     */
    pub fn compile_shaders(&mut self, device: &wgpu::Device, num_samples: u32) {
        self.shader_store.compile(device);

        let layout_desc = wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[
                &self.uniform_buffer.bind_group_layout,
                &self.geometry_uniform_buffer.bind_group_layout,
            ],
        };

        // now update all the GPU programs to use the latest code
        for (_, program) in self.programs.iter_mut() {
            program.update(
                device,
                &layout_desc,
                num_samples,
                &self.shader_store.shaders,
            );
        }
    }

    /**
     * Check if changes have been made to shaders and recompile if needed.
     * Call every timestep.
     */
    pub fn update_shaders(&mut self, device: &wgpu::Device, num_samples: u32) {
        // check for changes
        if let Ok(event) = self
            .changes_channel
            .recv_timeout(time::Duration::from_millis(1))
        {
            if let DebouncedEvent::Write(path) = event {
                let path_str = path.into_os_string().into_string().unwrap();
                println!("changes written to: {}", path_str);
                self.compile_shaders(device, num_samples);
            }
        }
    }

    /**
     * Update uniform data.
     * Call every timestep.
     */
    pub fn update_uniforms(&mut self) {
        self.uniforms.update();
    }

    /**
     * Fetch current GPU program.
     */
    pub fn current_pipeline(&self) -> Option<&wgpu::RenderPipeline> {
        self.programs
            .get(config::PROGRAMS[self.current_program])
            .as_ref()
            .unwrap()
            .pipeline
            .as_ref()
    }

    /**
     * Selects the current program
     * performs any housekeeping / initialization
     */
    pub fn select_program(&mut self, selected: usize) {
        if selected == self.current_program {
            return;
        }

        println!("program selected: {}", config::PROGRAMS[selected]);
        self.current_program = selected;
        self.uniforms.set_program_defaults(selected);
    }

    /**
     * Update GPU uniform buffer data with current uniforms.
     * Call in draw() before rendering.
     */
    pub fn update_uniform_buffers(
        &self,
        device: &wgpu::Device,
        encoder: &mut nannou::wgpu::CommandEncoder,
    ) {
        self.uniform_buffer
            .update::<uniforms::Data>(device, encoder, self.uniforms.as_bytes());

        self.geometry_uniform_buffer
            .update::<geometry_uniforms::Data>(device, encoder, self.geometry_uniforms.as_bytes());
    }

    /**
     * Fetch the appropriate bind groups to set positions for
     * the current program.
     * Call in draw() right before rendering.
     */
    pub fn get_bind_groups<'a>(&self) -> Vec<&wgpu::BindGroup> {
        let uniform_bind_group = &self.uniform_buffer.bind_group;
        let geometry_bind_group = &self.geometry_uniform_buffer.bind_group;
        vec![uniform_bind_group, geometry_bind_group]
    }
}
