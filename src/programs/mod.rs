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
mod uniform_buffer;
pub mod uniforms;

use crate::programs::uniforms::Bufferable;

pub type Programs = HashMap<String, program::Program>;

pub type UniformBuffers = HashMap<String, uniform_buffer::UniformBuffer>;

pub type ProgramUniforms = Vec<Vec<String>>;

/**
 * Stores GPU programs and related data
 */
pub struct ProgramStore {
    pub changes_channel: Receiver<DebouncedEvent>,
    pub current_program: usize,
    pub geometry_uniforms: geometry_uniforms::GeometryUniforms,
    pub programs: Programs,
    pub program_uniforms: ProgramUniforms,
    pub shader_store: shaders::ShaderStore,
    pub shader_watcher: notify::FsEventWatcher,
    pub uniforms: uniforms::Uniforms,
    pub uniform_buffers: UniformBuffers,
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

        // create the program map
        let mut programs = HashMap::new();
        for pipeline_desc in config::PIPELINES {
            let name = String::from(pipeline_desc[0]);
            programs.insert(
                name,
                program::Program::new(pipeline_desc[1], pipeline_desc[2]),
            );
        }

        // create uniform buffer map
        // TODO: encapsulate this in a UniformBufferStore struct?
        let mut uniform_buffers = HashMap::new();
        uniform_buffers.insert(String::from("general"), uniform_buffer);
        uniform_buffers.insert(String::from("geometry"), geometry_uniform_buffer);

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
            changes_channel,
            current_program: config::DEFAULT_PROGRAM,
            geometry_uniforms,
            programs,
            program_uniforms,
            shader_store: shaders::ShaderStore::new(),
            shader_watcher,
            uniforms,
            uniform_buffers,
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
            let uniform_buffers = &self.uniform_buffers;

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
     * Selects the current program performs any housekeeping / initialization
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
     * TODO: figure out a way to do this iteratively so that it can be left untouched when adding new buffers
     */
    pub fn update_uniform_buffers(
        &self,
        device: &wgpu::Device,
        encoder: &mut nannou::wgpu::CommandEncoder,
    ) {
        self.uniform_buffers
            .get("general")
            .unwrap()
            .update::<uniforms::Data>(device, encoder, self.uniforms.as_bytes());

        self.uniform_buffers
            .get("geometry")
            .unwrap()
            .update::<geometry_uniforms::Data>(device, encoder, self.geometry_uniforms.as_bytes());
    }

    /**
     * Fetch the appropriate bind groups to set positions for the current program.
     * Call in draw() right before rendering.
     */
    pub fn get_bind_groups<'a>(&self) -> Vec<&wgpu::BindGroup> {
        let program_uniforms = &self.program_uniforms[self.current_program];
        let bind_group_iter = program_uniforms
            .iter()
            .map(|u| &self.uniform_buffers.get(u).unwrap().bind_group);
        Vec::from_iter(bind_group_iter)
    }
}
