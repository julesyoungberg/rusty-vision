use nannou::prelude::*;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::mpsc::{channel, Receiver};
use std::time;

use crate::app_config;
use crate::util;

mod config;
pub mod program;
mod shaders;
pub mod uniforms;

pub type Programs = HashMap<String, program::Program>;

/// Stores GPU programs and related data.
/// Manages the maintenance of shader programs.
/// - listens to directory
/// - compiles code
/// - manages modules
/// - handles errors
/// - builds render pipelines
/// - manages uniform buffers
pub struct ProgramStore {
    pub buffer_store: uniforms::BufferStore,
    pub changes_channel: Receiver<DebouncedEvent>,
    pub current_program: usize,
    pub current_subscriptions: uniforms::UniformSubscriptions,
    pub programs: Programs,
    pub program_names: Vec<String>,
    pub program_defaults: HashMap<String, Option<config::ProgramDefaults>>,
    pub program_uniforms: HashMap<String, Vec<String>>,
    #[cfg(target_os = "macos")]
    pub shader_watcher: notify::FsEventWatcher,
    #[cfg(target_os = "linux")]
    pub shader_watcher: notify::INotifyWatcher,
    #[cfg(target_os = "windows")]
    pub shader_watcher: notify::ReadDirectoryChangesWatcher,
}

impl ProgramStore {
    pub fn new(app: &App, device: &wgpu::Device) -> Self {
        let program_config = config::get_config();

        let mut buffer_store = uniforms::BufferStore::new(device);

        // setup shader watcher
        let (send_channel, changes_channel) = channel();
        let mut shader_watcher = watcher(send_channel, time::Duration::from_secs(1)).unwrap();
        shader_watcher
            .watch(
                util::universal_path(app_config::SHADERS_PATH.to_string()).as_str(),
                RecursiveMode::Recursive,
            )
            .unwrap();

        // get program configuration
        let mut program_names = vec![];
        let mut programs = HashMap::new();
        let mut program_uniforms = HashMap::new();
        let mut program_defaults = HashMap::new();
        for (program_name, program_config) in program_config.programs {
            program_names.push(program_name.clone());

            programs.insert(
                program_name.clone(),
                program::Program::new(program_config.pipeline.vert, program_config.pipeline.frag),
            );

            program_uniforms.insert(program_name.clone(), program_config.uniforms);

            program_defaults.insert(program_name.clone(), program_config.defaults);
        }

        program_names.sort();

        let mut current_program = 0;
        for (index, program_name) in program_names.iter().enumerate() {
            if *program_name == program_config.default {
                current_program = index;
            }
        }

        let program_name = &program_names[current_program];

        let current_subscriptions =
            uniforms::get_subscriptions(&program_uniforms.get(program_name).unwrap());
        buffer_store.set_program_defaults(
            app,
            &current_subscriptions,
            &program_defaults.get(program_name).unwrap(),
        );

        Self {
            buffer_store,
            changes_channel,
            current_program,
            current_subscriptions,
            programs,
            program_names,
            program_defaults,
            program_uniforms,
            shader_watcher,
        }
    }

    /// Compile current program with latest shader code.
    /// Call once after initialization.
    pub fn compile_current(&mut self, device: &wgpu::Device, num_samples: u32) {
        // update the current GPU program to use the latest code
        let name = &self.program_names[self.current_program];
        let program = self.programs.get_mut(name).unwrap();
        let program_uniforms = &self.program_uniforms.get(name).unwrap();
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

    /// Check if changes have been made to shaders and recompile if needed.
    /// Call every timestep.
    pub fn update_shaders(&mut self, device: &wgpu::Device, num_samples: u32) {
        // check for changes
        if let Ok(event) = self.changes_channel.try_recv() {
            if let DebouncedEvent::Write(path) = event {
                let path_str = path.into_os_string().into_string().unwrap();
                println!("changes written to: {}", path_str);
                self.compile_current(device, num_samples);
            }
        }

        // check for a UI program change
        if self.current_program().is_new() {
            self.compile_current(device, num_samples);
        }

        // check for an image change
        if self.buffer_store.image_uniforms.updated {
            self.compile_current(device, num_samples);
            self.buffer_store.image_uniforms.updated = false;
        }
    }

    /// Update uniform data.
    /// Call every timestep.
    pub fn update_uniforms(&mut self, device: &wgpu::Device) {
        self.buffer_store
            .update(device, &self.current_subscriptions);
    }

    /// Fetch current GPU program.
    pub fn current_program(&self) -> &program::Program {
        self.programs
            .get(&self.program_names[self.current_program])
            .unwrap()
    }

    /// Fetch current GPU program.
    pub fn current_pipeline(&self) -> Option<&wgpu::RenderPipeline> {
        self.current_program().pipeline.as_ref()
    }

    /// Selects the current program performs any housekeeping / initialization
    pub fn select_program(&mut self, app: &App, selected: usize) {
        if selected == self.current_program {
            return;
        }

        let name = &self.program_names[selected];

        // first, clear the current program
        self.programs.get_mut(name).unwrap().clear();

        // next, update the current program and uniforms
        // it will be compiled in the next update()
        println!("program selected: {}", name);
        self.current_program = selected;
        self.current_subscriptions =
            uniforms::get_subscriptions(&self.program_uniforms.get(name).unwrap());
        self.buffer_store.set_program_defaults(
            app,
            &self.current_subscriptions,
            &self.program_defaults.get(name).unwrap(),
        );
    }

    /// Update GPU uniform buffers with current data.
    /// Call in draw() before rendering.
    pub fn update_uniform_buffers(
        &self,
        device: &wgpu::Device,
        encoder: &mut nannou::wgpu::CommandEncoder,
    ) {
        self.buffer_store
            .update_buffers(device, encoder, &self.current_subscriptions);
    }

    /// Fetch the appropriate bind groups to set positions for the current program.
    /// Call in draw() right before rendering.
    pub fn get_bind_groups<'a>(&self) -> Vec<&wgpu::BindGroup> {
        self.program_uniforms
            .get(&self.program_names[self.current_program])
            .unwrap()
            .iter()
            .map(|u| &self.buffer_store.buffers.get(u).unwrap().bind_group)
            .collect::<Vec<&wgpu::BindGroup>>()
    }

    pub fn errors(&self) -> &program::ProgramErrors {
        &self.current_program().errors
    }
}
