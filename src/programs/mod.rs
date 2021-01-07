use nannou::prelude::*;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver};
use std::time;

use crate::config;
use crate::pipelines;
use crate::uniforms;

mod shaders;

/**
 * Stores GPU programs and related data
 */
pub struct ProgramStore {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub changes_channel: Receiver<DebouncedEvent>,
    pub current_program: usize,
    pub pipelines: pipelines::Pipelines,
    pub shader_store: shaders::ShaderStore,
    pub shader_watcher: notify::FsEventWatcher,
    pub uniforms: uniforms::Uniforms,
    pub uniform_buffer: wgpu::Buffer,
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
        // setup uniform buffer
        let mut uniforms =
            uniforms::Uniforms::new(pt2(config::SIZE[0] as f32, config::SIZE[1] as f32));
        uniforms.set_program_defaults(config::DEFAULT_PROGRAM);
        let usage = wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST;
        let uniform_buffer = device.create_buffer_with_data(uniforms.as_bytes(), usage);

        // create bind group
        let bind_group_layout = wgpu::BindGroupLayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStage::FRAGMENT, false)
            .build(device);
        let bind_group = wgpu::BindGroupBuilder::new()
            .buffer::<uniforms::Data>(&uniform_buffer, 0..1)
            .build(device, &bind_group_layout);

        // setup shader watcher
        let (send_channel, changes_channel) = channel();
        let mut shader_watcher = watcher(send_channel, time::Duration::from_secs(1)).unwrap();
        shader_watcher
            .watch(config::SHADERS_PATH, RecursiveMode::Recursive)
            .unwrap();

        Self {
            bind_group,
            bind_group_layout,
            changes_channel,
            current_program: config::DEFAULT_PROGRAM,
            pipelines: HashMap::new(),
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
        let errors = self.shader_store.errors();
        let shader_modules = self.shader_store.shader_modules();

        if errors.keys().len() == 0 {
            self.pipelines = pipelines::create_pipelines(
                device,
                &self.bind_group_layout,
                num_samples,
                &shader_modules,
                &config::PIPELINES,
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
        self.uniforms.update_time();
    }

    /**
     * Fetch current GPU program.
     */
    pub fn current_pipeline(&self) -> Option<&wgpu::RenderPipeline> {
        self.pipelines.get(config::PROGRAMS[self.current_program])
    }

    /**
     * Update GPU uniform buffer data with current uniforms.
     * Call in draw() before rendering.
     */
    pub fn update_uniform_buffer(
        &self,
        device: &wgpu::Device,
        encoder: &mut nannou::wgpu::CommandEncoder,
    ) {
        let uniforms_size = std::mem::size_of::<uniforms::Data>() as wgpu::BufferAddress;
        let uniforms_bytes = self.uniforms.as_bytes();
        let uniforms_usage = wgpu::BufferUsage::COPY_SRC;
        let new_uniform_buffer = device.create_buffer_with_data(uniforms_bytes, uniforms_usage);
        encoder.copy_buffer_to_buffer(
            &new_uniform_buffer,
            0,
            &self.uniform_buffer,
            0,
            uniforms_size,
        );
    }
}
