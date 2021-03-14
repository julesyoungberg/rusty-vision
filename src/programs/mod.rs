use nannou::prelude::*;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::{channel, Receiver};
use std::time;

use crate::util;

mod config;
pub mod program;
mod shaders;
pub mod uniforms;

/// Stores GPU programs and related data.
/// Manages the maintenance of shader programs.
/// - listens to directory
/// - compiles code
/// - manages modules
/// - handles errors
/// - builds render pipelines
/// - manages uniform buffers
#[allow(dead_code)] // needed for shader_watcher
pub struct ProgramStore {
    pub buffer_store: uniforms::BufferStore,
    pub current_subscriptions: uniforms::UniformSubscriptions,
    pub folder_index: usize,
    pub folder_names: Vec<String>,
    pub program_names: Vec<String>,
    pub program_index: usize,

    changes_channel: Receiver<DebouncedEvent>,
    config: config::Config,
    current_program: Option<program::Program>,
    #[cfg(target_os = "macos")]
    shader_watcher: notify::FsEventWatcher,
    #[cfg(target_os = "linux")]
    shader_watcher: notify::INotifyWatcher,
    #[cfg(target_os = "windows")]
    shader_watcher: notify::ReadDirectoryChangesWatcher,
}

// TODO: handle errors better
impl ProgramStore {
    pub fn new(app: &App, device: &wgpu::Device, size: Vector2) -> Self {
        let config = config::get_config(app);

        let mut buffer_store = uniforms::BufferStore::new(device, size);

        // setup shader watcher
        let (send_channel, changes_channel) = channel();
        let mut shader_watcher = watcher(send_channel, time::Duration::from_secs(1)).unwrap();
        let shader_path = util::shaders_path_string(app);
        shader_watcher
            .watch(shader_path.as_str(), RecursiveMode::Recursive)
            .unwrap();

        // get folder configuration
        let mut folder_names = vec![];
        for (name, _) in config.folders.iter() {
            folder_names.push(name.clone());
        }

        folder_names.sort();
        let folder_index = folder_names
            .iter()
            .position(|n| *n == config.default)
            .unwrap();

        let folder_name = &folder_names[folder_index];
        let folder_config = config.folders.get(folder_name).unwrap();

        // get program configuration
        let mut program_names = vec![];
        for (name, _) in folder_config.programs.iter() {
            program_names.push(name.clone());
        }

        program_names.sort();
        let program_index = program_names
            .iter()
            .position(|n| *n == folder_config.default)
            .unwrap();

        let program_name = &program_names[program_index];
        let program_config = folder_config.programs.get(program_name).unwrap();
        let current_program = program::Program::new(program_config.clone(), folder_name.clone());

        let current_subscriptions = uniforms::get_subscriptions(&program_config.uniforms);
        buffer_store.set_program_defaults(
            app,
            device,
            &current_subscriptions,
            &program_config.defaults,
        );

        Self {
            buffer_store,
            changes_channel,
            config,
            current_program: Some(current_program),
            current_subscriptions,
            folder_index,
            folder_names,
            program_index,
            program_names,
            shader_watcher,
        }
    }

    /// Compile current program with latest shader code.
    /// Call once after initialization.
    pub fn compile_current(&mut self, app: &App, device: &wgpu::Device, num_samples: u32) {
        let current_program = match &mut self.current_program {
            Some(p) => p,
            None => {
                return;
            }
        };

        // update the current GPU program to use the latest code
        let buffers = &self.buffer_store.buffers;
        // map the current program's uniform list to a list of bind group layouts
        let bind_group_layouts = &current_program
            .config
            .uniforms
            .iter()
            .map(|u| &buffers.get(&u.to_string()).unwrap().bind_group_layout)
            .collect::<Vec<&wgpu::BindGroupLayout>>()[..];

        // update the program with the new shader code and appropriate layout description
        let layout_desc = wgpu::PipelineLayoutDescriptor { bind_group_layouts };
        current_program.compile(app, device, &layout_desc, num_samples);
    }

    /// Check if changes have been made to shaders and recompile if needed.
    /// Call every timestep.
    pub fn update_shaders(&mut self, app: &App, device: &wgpu::Device, num_samples: u32) {
        // check for changes
        if let Ok(event) = self.changes_channel.try_recv() {
            if let DebouncedEvent::Write(path) = event {
                let path_str = path.into_os_string().into_string().unwrap();
                println!("changes written to: {}", path_str);
                self.compile_current(app, device, num_samples);
            }
        }

        if let Some(current_program) = &mut self.current_program {
            if current_program.is_new()
                || self.buffer_store.image_uniforms.updated
                || self.buffer_store.webcam_uniforms.updated
            {
                self.compile_current(app, device, num_samples);

                if self.buffer_store.image_uniforms.updated {
                    self.buffer_store.image_uniforms.updated = false;
                }

                if self.buffer_store.webcam_uniforms.updated {
                    self.buffer_store.webcam_uniforms.updated = false;
                }
            }
        }
    }

    /// Update uniform data.
    /// Call every timestep.
    pub fn update_uniforms(&mut self, device: &wgpu::Device) {
        self.buffer_store
            .update(device, &self.current_subscriptions);
    }

    /// Fetch current GPU program.
    pub fn current_pipeline(&self) -> Option<&wgpu::RenderPipeline> {
        let current_program = &self.current_program.as_ref()?;
        current_program.pipeline.as_ref()
    }

    /// Selects the current program performs any housekeeping / initialization
    pub fn select_program(
        &mut self,
        app: &App,
        device: &wgpu::Device,
        selected: usize,
        force: bool,
    ) {
        if !force && selected == self.program_index {
            return;
        }

        let name = &self.program_names[selected];

        // first, clear the current program
        if let Some(current_program) = &mut self.current_program {
            current_program.clear();
        }

        // next, update the current program and uniforms
        // it will be compiled in the next update()
        println!("program selected: {}", name);
        self.program_index = selected;
        let folder_name = &self.folder_names[self.folder_index];
        let folder_config = self.config.folders.get(folder_name).unwrap();
        let program_config = folder_config.programs.get(name).unwrap();
        self.current_program = Some(program::Program::new(
            program_config.clone(),
            folder_name.clone(),
        ));
        self.current_subscriptions = uniforms::get_subscriptions(&program_config.uniforms);
        self.buffer_store.set_program_defaults(
            app,
            device,
            &self.current_subscriptions,
            &program_config.defaults,
        );
    }

    /// Selects the current shader folder
    pub fn select_folder(&mut self, app: &App, device: &wgpu::Device, selected: usize) {
        if selected == self.folder_index {
            return;
        }

        self.folder_index = selected;
        let name = &self.folder_names[selected];
        let folder_config = self.config.folders.get(name).unwrap();

        self.program_names = vec![];
        for (name, _) in folder_config.programs.iter() {
            self.program_names.push(name.clone());
        }

        self.program_names.sort();
        let program_index = self
            .program_names
            .iter()
            .position(|n| *n == folder_config.default)
            .unwrap();

        self.select_program(app, device, program_index, true);
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
    pub fn get_bind_groups(&self) -> Option<Vec<&wgpu::BindGroup>> {
        let current_program = self.current_program.as_ref()?;
        Some(
            current_program
                .config
                .uniforms
                .iter()
                .map(|u| &self.buffer_store.buffers.get(u).unwrap().bind_group)
                .collect::<Vec<&wgpu::BindGroup>>(),
        )
    }

    pub fn errors(&self) -> Option<&program::ProgramErrors> {
        let current_program = &self.current_program.as_ref()?;
        Some(&current_program.errors)
    }

    pub fn pause(&mut self) {
        self.buffer_store.pause(&self.current_subscriptions);
    }

    pub fn unpause(&mut self) {
        self.buffer_store.unpause(&self.current_subscriptions);
    }
}
