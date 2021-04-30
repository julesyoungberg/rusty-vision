use nannou::prelude::*;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::time;

use crate::programs::uniforms::base::Bufferable;
use crate::util;

mod config;
pub mod isf;
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
    pub current_subscriptions: Option<uniforms::UniformSubscriptions>,
    pub error: Option<String>,
    pub folder_index: usize,
    pub folder_names: Option<Vec<String>>,
    pub isf_pipeline: Option<isf::IsfPipeline>,
    pub isf_time: Option<isf::IsfTime>,
    pub program_names: Option<Vec<String>>,
    pub program_index: usize,

    changes_channel: Receiver<DebouncedEvent>,
    config: Option<config::Config>,
    current_program: Option<program::Program>,
    #[cfg(target_os = "macos")]
    shader_watcher: notify::FsEventWatcher,
    #[cfg(target_os = "linux")]
    shader_watcher: notify::INotifyWatcher,
    #[cfg(target_os = "windows")]
    shader_watcher: notify::ReadDirectoryChangesWatcher,
    render_texture: wgpu::Texture,
    texture_reshaper: wgpu::TextureReshaper,
}

impl ProgramStore {
    pub fn new(app: &App, device: &wgpu::Device, size: Vector2, num_samples: u32) -> Self {
        let buffer_store = uniforms::BufferStore::new(device, size);

        // setup shader watcher
        let (send_channel, changes_channel) = channel();
        let mut shader_watcher = watcher(send_channel, time::Duration::from_secs(1)).unwrap();
        let shader_path = util::shaders_path_string(app);
        shader_watcher
            .watch(shader_path.as_str(), RecursiveMode::Recursive)
            .unwrap();

        let render_texture = util::create_app_texture(device, size, num_samples);
        let texture_reshaper = util::create_texture_reshaper(device, &render_texture, num_samples);

        Self {
            buffer_store,
            changes_channel,
            config: None,
            current_program: None,
            current_subscriptions: None,
            error: None,
            folder_index: 0,
            folder_names: None,
            isf_pipeline: None,
            isf_time: None,
            program_index: 0,
            program_names: None,
            shader_watcher,
            render_texture,
            texture_reshaper,
        }
    }

    fn get_folder_name(&self) -> Option<String> {
        let folder_names = &self.folder_names.as_ref()?;
        Some(folder_names[self.folder_index].clone())
    }

    fn get_program_name(&self) -> Option<String> {
        let program_names = &self.program_names.as_ref()?;
        Some(program_names[self.program_index].clone())
    }

    /// Create the render pipeline with the program's required buffers
    fn create_render_pipeline(&mut self, device: &wgpu::Device, num_samples: u32) {
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
            .as_ref()
            .unwrap()
            .iter()
            .map(|u| &buffers.get(&u.to_string()).unwrap().bind_group_layout)
            .collect::<Vec<&wgpu::BindGroupLayout>>()[..];
        // update the program with the new shader code and appropriate layout description
        let layout_desc = wgpu::PipelineLayoutDescriptor { bind_group_layouts };
        current_program.create_render_pipeline(device, &layout_desc, num_samples);
    }

    /// Compile current program with latest shader code.
    /// Call once after initialization.
    fn compile_current(&mut self, app: &App, device: &wgpu::Device, num_samples: u32) {
        let current_program = match &mut self.current_program {
            Some(p) => p,
            None => {
                return;
            }
        };

        current_program.compile(app, device);
        self.create_render_pipeline(device, num_samples);
    }

    fn configure_isf_program(
        &mut self,
        app: &App,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        program_config: &config::ProgramConfig,
        folder_name: String,
        num_samples: u32,
        size: Point2,
    ) {
        let shader_path = app
            .project_path()
            .unwrap()
            .join("shaders")
            .join(folder_name)
            .join(program_config.pipeline.frag.clone());

        let media_path = app.project_path().unwrap().join("media");

        let isf_pipeline = isf::IsfPipeline::new(
            device,
            encoder,
            None,
            shader_path,
            Frame::TEXTURE_FORMAT,
            [size[0] as u32, size[1] as u32],
            num_samples,
            &media_path,
            num_samples,
        );

        let isf_time = Default::default();

        self.isf_pipeline = Some(isf_pipeline);
        self.isf_time = Some(isf_time);
        self.error = None;
        self.current_program = None;
        self.current_subscriptions = None;
    }

    fn configure_program(
        &mut self,
        app: &App,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        program_config: &config::ProgramConfig,
        folder_name: String,
        num_samples: u32,
        size: Point2,
    ) {
        if let Some(_) = self.current_subscriptions {
            self.buffer_store.end_session();
        }

        if let Some(ref mut isf_pipeline) = self.isf_pipeline {
            isf_pipeline.end_session();
        }

        if let Some(isf) = program_config.isf {
            if isf {
                self.configure_isf_program(
                    app,
                    device,
                    encoder,
                    program_config,
                    folder_name,
                    num_samples,
                    size,
                );
                return;
            }
        }

        self.isf_pipeline = None;
        self.isf_time = None;

        // create current program
        let current_program = program::Program::new(program_config.clone(), folder_name);
        self.current_program = Some(current_program);

        // get subscriptions and initialize
        let current_subscriptions =
            uniforms::get_subscriptions(&program_config.uniforms.as_ref().unwrap());
        self.buffer_store.configure(
            app,
            device,
            encoder,
            &current_subscriptions,
            &program_config.config,
            size,
            num_samples,
        );

        self.current_subscriptions = Some(current_subscriptions);
        self.compile_current(app, device, num_samples);
        self.error = None;
    }

    /// Read fresh config and recompile
    pub fn configure(
        &mut self,
        app: &App,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        num_samples: u32,
        size: Point2,
    ) {
        // first, clear the current program
        if let Some(current_program) = &mut self.current_program {
            current_program.clear();
        }

        let config = match config::get_config(app) {
            Ok(c) => c,
            Err(e) => {
                self.error = Some(e);
                return;
            }
        };

        self.config = Some(config.clone());
        let folder_names = config.get_folder_names();

        let old_folder_name_opt = self.get_folder_name();

        self.folder_names = Some(folder_names.clone());
        let new_folder_name_opt = self.get_folder_name();

        let mut folder_name_opt: Option<String> = None;
        let mut using_defaults = false;

        // if the name matches with new config and old then use the old folder name
        if let Some(old_folder_name) = old_folder_name_opt {
            if let Some(new_folder_name) = new_folder_name_opt {
                if old_folder_name == new_folder_name {
                    folder_name_opt = Some(old_folder_name);
                }
            }
        }

        // if that didn't work read the default folder
        if folder_name_opt.is_none() {
            let folder_index = match config.get_default_folder_index(&folder_names) {
                Ok(i) => i,
                Err(e) => {
                    self.error = Some(e);
                    return;
                }
            };
            self.folder_index = folder_index;
            folder_name_opt = Some(folder_names[folder_index].clone());
            using_defaults = true;
        }

        let mut folder_name = folder_name_opt.unwrap();

        let missing_folder = format!("Missing default folder config '{}'", config.default);

        // read the folder config, try the default folder on failure
        let folder_config = match config.folders.get(&folder_name) {
            Some(c) => c,
            None => {
                // we already tried the new config, abort
                if using_defaults {
                    self.error = Some(missing_folder);
                    return;
                }

                // maybe the config updated, get the new default index
                let folder_index = match config.get_default_folder_index(&folder_names) {
                    Ok(i) => i,
                    Err(e) => {
                        self.error = Some(e);
                        return;
                    }
                };

                self.folder_index = folder_index;
                folder_name = folder_names[folder_index].clone();
                using_defaults = true;

                // retry
                match config.folders.get(&folder_name) {
                    Some(c) => c,
                    None => {
                        self.error = Some(missing_folder);
                        return;
                    }
                }
            }
        };

        let program_names = folder_config.get_program_names();
        let mut program_name_opt: Option<String> = None;

        // try to use the old program if it is still valid
        if !using_defaults {
            let old_program_name_opt = self.get_program_name();
            self.program_names = Some(program_names.clone());
            let new_program_name_opt = self.get_program_name();

            if let Some(old_program_name) = old_program_name_opt {
                if let Some(new_program_name) = new_program_name_opt {
                    if old_program_name == new_program_name {
                        program_name_opt = Some(old_program_name);
                    }
                }
            }
        } else {
            self.program_names = Some(program_names.clone());
        }

        // fallback on the default program
        if program_name_opt.is_none() {
            let program_index = match folder_config.get_default_program_index(&program_names) {
                Ok(i) => i,
                Err(e) => {
                    self.error = Some(e);
                    return;
                }
            };

            self.program_index = program_index;
            program_name_opt = Some(folder_config.default.clone());
            using_defaults = true;
        }

        let mut program_name = program_name_opt.unwrap();
        let missing_program = format!("Missing default program config '{}'", folder_config.default);

        // read the program config, falling back on the default on failure
        let program_config = match folder_config.programs.get(&program_name) {
            Some(c) => c,
            None => {
                // already using defaults, abort
                if using_defaults {
                    self.error = Some(missing_program);
                    return;
                }

                // maybe the config updated, get the new default index
                let program_index = match folder_config.get_default_program_index(&program_names) {
                    Ok(i) => i,
                    Err(e) => {
                        self.error = Some(e);
                        return;
                    }
                };

                self.program_index = program_index;
                program_name = program_names[program_index].clone();

                // retry
                match folder_config.programs.get(&program_name) {
                    Some(c) => c,
                    None => {
                        self.error = Some(missing_program);
                        return;
                    }
                }
            }
        };

        self.configure_program(
            app,
            device,
            encoder,
            program_config,
            folder_name,
            num_samples,
            size,
        );
    }

    fn path_changed(&mut self) -> Option<PathBuf> {
        match self.changes_channel.try_recv() {
            Ok(event) => match event {
                DebouncedEvent::Write(path) => Some(path),
                _ => None,
            },
            _ => None,
        }
    }

    /// Check if changes have been made to shaders and recompile if needed.
    /// Call every timestep.
    fn update_shaders(
        &mut self,
        app: &App,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        size: Point2,
        num_samples: u32,
        path_changed: Option<PathBuf>,
    ) {
        if let Some(path) = path_changed {
            let path_str = path.into_os_string().into_string().unwrap();
            println!("changes written to: {}", path_str);

            if path_str.ends_with(".json") {
                self.configure(app, device, encoder, num_samples, size);
            } else {
                self.compile_current(app, device, num_samples);
            }
        }

        if let Some(current_program) = &mut self.current_program {
            if current_program.is_new() {
                // if the shader has changed recompile and recreate the pipeline
                self.compile_current(app, device, num_samples);
                self.buffer_store.finish_update();
            } else if self.buffer_store.updated() {
                // if the data has changed only just recreated the pipeline
                self.create_render_pipeline(device, num_samples);
                self.buffer_store.finish_update();
            }
        }
    }

    /// Update uniform data.
    /// Call every timestep.
    fn update_uniforms(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        size: Point2,
        num_samples: u32,
    ) {
        if let Some(current_subscriptions) = self.current_subscriptions.as_ref() {
            self.buffer_store
                .update(device, encoder, current_subscriptions, size, num_samples);
        }
    }

    /// Update data and shaders.
    pub fn encode_update(
        &mut self,
        app: &App,
        update: Update,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        size: Point2,
        num_samples: u32,
    ) {
        let path_changed = self.path_changed();

        if let Some(isf_pipeline) = self.isf_pipeline.as_mut() {
            let mut touched: Vec<String> = vec![];
            if let Some(path) = path_changed.clone() {
                touched.push(String::from(path.to_str().unwrap()));
            }

            let images_path = app.project_path().unwrap().join("media");
            isf_pipeline.encode_update(device, encoder, &images_path, touched, num_samples);

            if let Some(isf_time) = self.isf_time.as_mut() {
                isf_time.time = update.since_start.secs() as _;
                isf_time.time_delta = update.since_last.secs() as _;
            }
        } else {
            self.update_uniforms(device, encoder, size, num_samples);
        }

        self.update_shaders(app, device, encoder, size, num_samples, path_changed);
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
        encoder: &mut wgpu::CommandEncoder,
        selected: usize,
        force: bool,
        size: Point2,
        num_samples: u32,
    ) -> Option<bool> {
        if self.error.is_none() && !force && selected == self.program_index {
            return None;
        }

        let program_names = &self.program_names.as_ref()?;
        let name = &program_names[selected];

        // first, clear the current program
        if let Some(current_program) = &mut self.current_program {
            current_program.clear();
        }

        // next, update the current program and uniforms
        // it will be compiled in the next update()
        println!("program selected: {}", name);
        self.program_index = selected;

        let folder_name = self.get_folder_name()?;
        let config = self.config.clone()?;
        let folder_config = config.folders.get(&folder_name).unwrap();

        let program_config = match folder_config.programs.get(name) {
            Some(c) => c,
            None => {
                self.error = Some(format!("Missing program config '{}'", name));
                return None;
            }
        };

        self.configure_program(
            app,
            device,
            encoder,
            program_config,
            folder_name,
            num_samples,
            size,
        );

        Some(true)
    }

    /// Selects the current shader folder
    pub fn select_folder(
        &mut self,
        app: &App,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        selected: usize,
        size: Point2,
        num_samples: u32,
    ) -> Option<bool> {
        if self.error.is_none() && selected == self.folder_index {
            return None;
        }

        self.folder_index = selected;
        let folder_names = &self.folder_names.as_ref()?;
        let name = &folder_names[selected];
        let config = &self.config.as_ref()?;
        let folder_config = match config.folders.get(name) {
            Some(c) => c,
            None => {
                self.error = Some(format!("Missing folder config '{}'", name));
                return None;
            }
        };

        let mut program_names = vec![];
        for (name, _) in folder_config.programs.iter() {
            program_names.push(name.clone());
        }
        program_names.sort();

        let program_index = match program_names
            .iter()
            .position(|n| *n == folder_config.default)
        {
            Some(i) => i,
            None => {
                self.error = Some(format!(
                    "Invalid default program '{}'",
                    folder_config.default
                ));
                return None;
            }
        };

        self.program_names = Some(program_names);
        self.select_program(app, device, encoder, program_index, true, size, num_samples)
    }

    /// Update GPU uniform buffers with current data.
    /// Call in draw() before rendering.
    pub fn update_uniform_buffers(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        if let Some(current_subscriptions) = self.current_subscriptions.as_ref() {
            self.buffer_store
                .update_buffers(device, encoder, current_subscriptions);
        }
    }

    /// Fetch the appropriate bind groups to set positions for the current program.
    /// Call in draw() right before rendering.
    pub fn get_bind_groups(&self) -> Option<Vec<&wgpu::BindGroup>> {
        let current_program = self.current_program.as_ref()?;
        Some(
            current_program
                .config
                .uniforms
                .as_ref()
                .unwrap()
                .iter()
                .map(|u| &self.buffer_store.buffers.get(u).unwrap().bind_group)
                .collect::<Vec<&wgpu::BindGroup>>(),
        )
    }

    pub fn get_program_errors(&self) -> Option<program::ProgramErrors> {
        if let Some(ref isf_pipeline) = self.isf_pipeline {
            return isf_pipeline.get_program_errors();
        }

        let current_program = &self.current_program.as_ref()?;
        Some(current_program.errors.clone())
    }

    pub fn get_data_errors(&self) -> HashMap<String, Vec<String>> {
        if let Some(ref isf_pipeline) = self.isf_pipeline {
            return isf_pipeline.get_data_errors();
        }

        self.buffer_store.get_errors()
    }

    pub fn pause(&mut self) {
        if let Some(current_subscriptions) = &self.current_subscriptions {
            self.buffer_store.pause(current_subscriptions);
        }

        if let Some(ref mut isf_pipeline) = self.isf_pipeline {
            isf_pipeline.pause();
        }
    }

    pub fn unpause(&mut self) {
        if let Some(current_subscriptions) = &self.current_subscriptions {
            self.buffer_store.unpause(current_subscriptions);
        }

        if let Some(ref mut isf_pipeline) = self.isf_pipeline {
            isf_pipeline.unpause();
        }
    }

    pub fn is_multipass(&self) -> bool {
        if let Some(ref subscriptions) = self.current_subscriptions {
            return subscriptions.multipass;
        }

        if let Some(ref isf_pipeline) = self.isf_pipeline {
            return !isf_pipeline.isf_data.passes().is_empty();
        }

        false
    }

    pub fn num_passes(&mut self) -> u32 {
        if let Some(ref isf_pipeline) = self.isf_pipeline {
            isf_pipeline.isf_data.passes().len() as u32
        } else {
            self.buffer_store.multipass_uniforms.passes as u32
        }
    }

    pub fn reset_pass_index(&mut self) {
        if let Some(ref mut isf_pipeline) = self.isf_pipeline {
            isf_pipeline.pass_index = 0;
        } else {
            self.buffer_store.multipass_uniforms.data.pass_index = 0;
        }
    }

    pub fn increment_pass_index(&mut self) {
        if let Some(ref mut isf_pipeline) = self.isf_pipeline {
            isf_pipeline.pass_index += 1;
        } else {
            self.buffer_store.multipass_uniforms.data.pass_index += 1;
        }
    }

    pub fn multipass_textures(&self) -> Vec<&wgpu::Texture> {
        if let Some(ref isf_pipeline) = self.isf_pipeline {
            isf_pipeline
                .isf_data
                .passes()
                .iter()
                .map(|pass| &pass.uniform_texture)
                .collect::<Vec<&wgpu::Texture>>()
        } else {
            self.buffer_store.multipass_uniforms.textures()
        }
    }

    pub fn get_render_texture(&self, index: usize) -> &wgpu::Texture {
        if let Some(isf_pipeline) = &self.isf_pipeline {
            return isf_pipeline.get_render_texture(index);
        }

        &self.render_texture
    }

    pub fn get_texture_reshaper(&self) -> &wgpu::TextureReshaper {
        if let Some(isf_pipeline) = &self.isf_pipeline {
            if let Some(texture_reshaper) = isf_pipeline.get_texture_reshaper() {
                return texture_reshaper;
            }
        }

        &self.texture_reshaper
    }
}
