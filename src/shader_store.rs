use nannou::prelude::*;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver};
use std::time;

use crate::config;
use crate::pipelines;
use crate::shaders;
use crate::uniforms;

/**
 * Stores shader source code
 */
pub struct ShaderStore {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub changes_channel: Receiver<DebouncedEvent>,
    pub compilation_errors: shaders::CompilationErrors,
    pub current_program: usize,
    pub pipelines: pipelines::Pipelines,
    pub shader_watcher: notify::FsEventWatcher,
    pub uniforms: uniforms::Uniforms,
    pub uniform_buffer: wgpu::Buffer,
}

/**
 * Manages the maintenance (listening and loading) of shader code
 */
impl ShaderStore {
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
            compilation_errors: HashMap::new(),
            current_program: config::DEFAULT_PROGRAM,
            pipelines: HashMap::new(),
            shader_watcher,
            uniforms,
            uniform_buffer,
        }
    }

    pub fn compile_shaders(&mut self, device: &wgpu::Device, num_samples: u32) {
        let compilation_result = shaders::compile_shaders(device, config::SHADERS);

        if compilation_result.errors.keys().len() == 0 {
            self.pipelines = pipelines::create_pipelines(
                device,
                &self.bind_group_layout,
                num_samples,
                &compilation_result.shaders,
                &config::PIPELINES,
            );
        }

        self.compilation_errors = compilation_result.errors;
    }

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

    pub fn update_uniforms(&mut self) {
        self.uniforms.update_time();
    }

    pub fn current_pipeline(&self) -> Option<&wgpu::RenderPipeline> {
        self.pipelines.get(config::PROGRAMS[self.current_program])
    }

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
