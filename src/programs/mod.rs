use nannou::prelude::*;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::mpsc::{channel, Receiver};
use std::time;

use crate::config;

pub mod geometry_uniforms;
mod program;
mod shaders;
mod uniform_buffers;
pub mod uniforms;

pub type Programs = HashMap<String, program::Program>;

pub type ProgramUniforms = Vec<Vec<String>>;

/**
 * Stores GPU programs and related data
 */
pub struct ProgramStore {
    pub buffer_store: uniform_buffers::UniformBufferStore,
    pub changes_channel: Receiver<DebouncedEvent>,
    pub current_program: usize,
    pub programs: Programs,
    pub program_uniforms: ProgramUniforms,
    pub shader_store: shaders::ShaderStore,
    pub shader_watcher: notify::FsEventWatcher,
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
        let buffer_store = uniform_buffers::UniformBufferStore::new(device);

        // setup shader watcher
        let (send_channel, changes_channel) = channel();
        let mut shader_watcher = watcher(send_channel, time::Duration::from_secs(1)).unwrap();
        shader_watcher
            .watch(config::SHADERS_PATH, RecursiveMode::Recursive)
            .unwrap();

        // create the program map
        let mut programs = HashMap::new();
        for pipeline_desc in config::PIPELINES {
            let name = String::from(pipeline_desc[0]);
            programs.insert(
                name,
                program::Program::new(pipeline_desc[1], pipeline_desc[2]),
            );
        }

        // parse the uniform configuration into a vector of vector of strings for easy lookup & iteration
        let program_uniforms = config::PROGRAM_UNIFORMS
            .iter()
            .map(|u| {
                u.split(",")
                    .map(|s| String::from(s))
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>();
        println!("program_uniforms: {:?}", program_uniforms);

        Self {
            buffer_store,
            changes_channel,
            current_program: config::DEFAULT_PROGRAM,
            programs,
            program_uniforms,
            shader_store: shaders::ShaderStore::new(),
            shader_watcher,
        }
    }

    /**
     * Compile all shaders and [re]create pipelines.
     * Call once after initialization.
     * TODO: Potential optimization: only compile the shaders needed for the current program,
     * then only update the current program
     */
    pub fn compile_shaders(&mut self, device: &wgpu::Device, num_samples: u32) {
        self.shader_store.compile(device);

        // now update all the GPU program's to use the latest code
        for (i, p) in self.programs.iter_mut().enumerate() {
            let program_uniforms = &self.program_uniforms[i];
            let uniform_buffers = &self.buffer_store.buffers;

            // map the current program's uniform list to a list of bind group layouts
            let bind_group_layout_iter = program_uniforms.iter().map(|u| {
                &uniform_buffers
                    .get(&u.to_string())
                    .unwrap()
                    .bind_group_layout
            });

            // update the program with the new shader code and appropriate layout description
            let bind_group_layouts = &Vec::from_iter(bind_group_layout_iter)[..];
            let layout_desc = wgpu::PipelineLayoutDescriptor { bind_group_layouts };
            p.1.update(
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
        self.buffer_store.update();
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
     * Selects the current program performs any housekeeping / initialization
     */
    pub fn select_program(&mut self, selected: usize) {
        if selected == self.current_program {
            return;
        }

        println!("program selected: {}", config::PROGRAMS[selected]);
        self.current_program = selected;
        self.buffer_store.set_program_defaults(selected);
    }

    /**
     * Update GPU uniform buffers with current data.
     * Call in draw() before rendering.
     */
    pub fn update_uniform_buffers(
        &self,
        device: &wgpu::Device,
        encoder: &mut nannou::wgpu::CommandEncoder,
    ) {
        self.buffer_store.update_buffers(device, encoder);
    }

    /**
     * Fetch the appropriate bind groups to set positions for the current program.
     * Call in draw() right before rendering.
     */
    pub fn get_bind_groups<'a>(&self) -> Vec<&wgpu::BindGroup> {
        let program_uniforms = &self.program_uniforms[self.current_program];
        let bind_group_iter = program_uniforms
            .iter()
            .map(|u| &self.buffer_store.buffers.get(u).unwrap().bind_group);
        Vec::from_iter(bind_group_iter)
    }
}
