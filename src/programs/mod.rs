use nannou::prelude::*;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::mpsc::{channel, Receiver};
use std::time;

use crate::config;

pub mod program;
mod shaders;
pub mod uniforms;

pub type Programs = HashMap<String, program::Program>;

pub type ProgramUniforms = Vec<Vec<String>>;

/**
 * Stores GPU programs and related data
 */
pub struct ProgramStore {
    pub buffer_store: uniforms::BufferStore,
    pub changes_channel: Receiver<DebouncedEvent>,
    pub current_program: usize,
    pub programs: Programs,
    pub program_uniforms: ProgramUniforms,
    #[cfg(target_os = "macos")]
    pub shader_watcher: notify::FsEventWatcher,
    #[cfg(target_os = "linux")]
    pub shader_watcher: notify::INotifyWatcher,
    #[cfg(target_os = "windows")]
    pub shader_watcher: notify::ReadDirectoryChangesWatcher,
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
        let buffer_store = uniforms::BufferStore::new(device);

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

        Self {
            buffer_store,
            changes_channel,
            current_program: config::DEFAULT_PROGRAM,
            programs,
            program_uniforms,
            shader_watcher,
        }
    }

    /**
     * Compile current program with latest shader code.
     * Call once after initialization.
     */
    pub fn compile_current(&mut self, device: &wgpu::Device, num_samples: u32) {
        // update the current GPU program to use the latest code
        let name = config::PROGRAMS[self.current_program];
        let program = self.programs.get_mut(name).unwrap();
        let program_uniforms = &self.program_uniforms[self.current_program];
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
        program.compile(device, &layout_desc, num_samples);
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
                self.compile_current(device, num_samples);
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
    pub fn current_program(&self) -> &program::Program {
        self.programs
            .get(config::PROGRAMS[self.current_program])
            .unwrap()
    }

    /**
     * Fetch current GPU program.
     */
    pub fn current_pipeline(&self) -> Option<&wgpu::RenderPipeline> {
        self.current_program().pipeline.as_ref()
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

    pub fn errors(&self) -> &program::ProgramErrors {
        &self.current_program().errors
    }
}
