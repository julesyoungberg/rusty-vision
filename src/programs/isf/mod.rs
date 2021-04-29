// a fork of https://github.com/nannou-org/nannou/blob/master/nannou_isf/src/pipeline.rs

use nannou::prelude::*;
use nannou::ui::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use threadpool::ThreadPool;

use crate::programs::uniforms::audio_source::AudioSource;

pub mod data;
mod shader;
mod util;

#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

const VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
];

/// Timing information passed into the shader.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct IsfTime {
    /// The time since the start of the application.
    pub time: f32,
    /// The time since the last frame was rendered.
    pub time_delta: f32,
    /// The date as year, month, day and seconds.
    pub date: [f32; 4],
    /// The current frame that is to be rendered.
    pub frame_index: i32,
}

/// A render pipeline designed for hotloading!
pub struct IsfPipeline {
    pub isf: Option<isf::Isf>,
    pub isf_data: data::IsfData,
    pub widget_ids: Option<HashMap<String, widget::Id>>,
    pub isf_err: Option<util::IsfError>,
    pub image_loader: data::ImageLoader,
    pub updated: bool,
    pub pass_index: u32,
    audio_source: AudioSource,
    vs: shader::Shader,
    fs: shader::Shader,
    sampler: wgpu::Sampler,
    isf_uniform_buffer: wgpu::Buffer,
    isf_bind_group_layout: wgpu::BindGroupLayout,
    isf_bind_group: wgpu::BindGroup,
    isf_inputs_uniform_buffer: Option<wgpu::Buffer>,
    isf_inputs_bind_group_layout: Option<wgpu::BindGroupLayout>,
    isf_inputs_bind_group: Option<wgpu::BindGroup>,
    isf_textures_bind_group_layout: wgpu::BindGroupLayout,
    isf_textures_bind_group: wgpu::BindGroup,
    layout: wgpu::PipelineLayout,
    render_pipeline: Option<wgpu::RenderPipeline>,
    vertex_buffer: wgpu::Buffer,
    dst_format: wgpu::TextureFormat,
    dst_texture_size: [u32; 2],
    dst_sample_count: u32,
    texture_reshaper: Option<wgpu::TextureReshaper>,
}

fn isf_uniforms_as_bytes(data: &data::IsfUniforms) -> &[u8] {
    unsafe { wgpu::bytes::from(data) }
}

fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}

// Includes the sampler and then all textures for all images and passes.
fn create_isf_textures_bind_group_layout(
    device: &wgpu::Device,
    isf_data: &data::IsfData,
) -> wgpu::BindGroupLayout {
    // Begin with the sampler.
    let mut builder = wgpu::BindGroupLayoutBuilder::new().sampler(wgpu::ShaderStage::FRAGMENT);
    for texture in data::isf_data_textures(isf_data) {
        builder = builder.sampled_texture(
            wgpu::ShaderStage::FRAGMENT,
            false,
            wgpu::TextureViewDimension::D2,
            texture.component_type(),
        );
    }
    builder.build(device)
}

fn create_isf_textures_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    sampler: &wgpu::Sampler,
    isf_data: &data::IsfData,
) -> wgpu::BindGroup {
    let mut builder = wgpu::BindGroupBuilder::new().sampler(sampler);
    let texture_views: Vec<_> = data::isf_data_textures(isf_data)
        .map(|tex| tex.view().build())
        .collect();
    for texture_view in &texture_views {
        builder = builder.texture_view(texture_view);
    }
    builder.build(device, layout)
}

fn create_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::PipelineLayout {
    let desc = wgpu::PipelineLayoutDescriptor { bind_group_layouts };
    device.create_pipeline_layout(&desc)
}

fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    vs_mod: &wgpu::ShaderModule,
    fs_mod: &wgpu::ShaderModule,
    dst_format: wgpu::TextureFormat,
    sample_count: u32,
) -> wgpu::RenderPipeline {
    wgpu::RenderPipelineBuilder::from_layout(layout, &vs_mod)
        .fragment_shader(fs_mod)
        .color_format(dst_format)
        .add_vertex_buffer::<Vertex>(&wgpu::vertex_attr_array![0 => Float2])
        .sample_count(sample_count)
        .primitive_topology(wgpu::PrimitiveTopology::TriangleStrip)
        .build(device)
}

fn build_uniform_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    wgpu::BindGroupLayoutBuilder::new()
        .uniform_buffer(wgpu::ShaderStage::FRAGMENT, false)
        .build(device)
}

impl IsfPipeline {
    pub fn new(
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        vs_path: Option<PathBuf>,
        fs_path: PathBuf,
        dst_format: wgpu::TextureFormat,
        dst_texture_size: [u32; 2],
        dst_sample_count: u32,
        images_path: &Path,
        num_samples: u32,
    ) -> Self {
        let isf_res = util::read_isf_from_path(&fs_path);
        let (isf, error) = util::split_result(isf_res);

        // Create the shaders
        let fs = shader::Shader::fragment_from_path(device, fs_path);
        let vs = match vs_path {
            None => shader::Shader::vertex_default(device),
            Some(vs_path) => shader::Shader::vertex_from_path(device, vs_path),
        };

        dbg!(&vs);
        dbg!(&fs);

        // Create a threadpool for loading images.
        let threadpool = ThreadPool::default();
        let image_loader = data::ImageLoader { threadpool };

        let mut audio_source = AudioSource::new();

        // Initialise the ISF imported images, input data and passes
        let mut isf_data = data::IsfData::default();
        if let Some(ref isf) = isf {
            data::sync_isf_data(
                device,
                encoder,
                isf,
                dst_texture_size,
                &image_loader,
                &images_path,
                &mut audio_source,
                &mut isf_data,
                num_samples,
            );
        }

        // Prepare the uniform buffers.
        let uniforms_usage = wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST;
        let [dst_tex_w, dst_tex_h] = dst_texture_size;
        let isf_uniforms = data::IsfUniforms {
            pass_index: 0,
            render_size: [dst_tex_w as f32, dst_tex_h as f32],
            time: 0.0,
            time_delta: 0.0,
            date: [0.0; 4],
            frame_index: 0,
        };

        let mut bind_group_layouts = vec![];

        println!("creating isf uniforms");
        let isf_uniforms_bytes = isf_uniforms_as_bytes(&isf_uniforms);
        let isf_uniform_buffer =
            device.create_buffer_with_data(&isf_uniforms_bytes, uniforms_usage);
        let isf_bind_group_layout = build_uniform_bind_group_layout(device);
        bind_group_layouts.push(&isf_bind_group_layout);
        let isf_bind_group = wgpu::BindGroupBuilder::new()
            .buffer::<data::IsfUniforms>(&isf_uniform_buffer, 0..1)
            .build(device, &isf_bind_group_layout);

        println!("creating isf textures");
        let isf_textures_bind_group_layout =
            create_isf_textures_bind_group_layout(device, &isf_data);
        bind_group_layouts.push(&isf_textures_bind_group_layout);
        let sampler = wgpu::SamplerBuilder::new().build(device);
        let isf_textures_bind_group = create_isf_textures_bind_group(
            device,
            &isf_textures_bind_group_layout,
            &sampler,
            &isf_data,
        );

        println!("creating isf input uniforms");
        let isf_input_uniforms_bytes_vec = data::get_isf_input_uniforms_bytes_vec(&isf, &isf_data);
        let isf_input_uniforms_bytes = &isf_input_uniforms_bytes_vec[..];
        let mut isf_inputs_uniform_buffer = None;
        let mut isf_inputs_bind_group_layout = None;
        let mut isf_inputs_bind_group = None;

        if !isf_input_uniforms_bytes.is_empty() {
            let uniform_buffer =
                device.create_buffer_with_data(&isf_input_uniforms_bytes, uniforms_usage);
            let bind_group_layout = build_uniform_bind_group_layout(device);
            let bind_group = wgpu::BindGroupBuilder::new()
                .buffer_bytes(&uniform_buffer, 0..1)
                .build(device, &bind_group_layout);

            isf_inputs_uniform_buffer = Some(uniform_buffer);
            isf_inputs_bind_group_layout = Some(bind_group_layout);
            isf_inputs_bind_group = Some(bind_group);
            bind_group_layouts.push(isf_inputs_bind_group_layout.as_ref().unwrap());
        }

        // Create the render pipeline.
        let layout = create_pipeline_layout(device, &bind_group_layouts);
        let render_pipeline = match (vs.module.as_ref(), fs.module.as_ref()) {
            (Some(vs_mod), Some(fs_mod)) => Some(create_render_pipeline(
                device,
                &layout,
                vs_mod,
                fs_mod,
                dst_format,
                dst_sample_count,
            )),
            _ => None,
        };

        // The quad vertex buffer.
        let vertices_bytes = vertices_as_bytes(&VERTICES[..]);
        let vertex_usage = wgpu::BufferUsage::VERTEX;
        let vertex_buffer = device.create_buffer_with_data(vertices_bytes, vertex_usage);

        let texture_reshaper = match isf_data.get_final_texture() {
            Some(texture) => Some(crate::util::create_texture_reshaper(
                device,
                &texture,
                num_samples,
            )),
            None => None,
        };

        Self {
            isf,
            isf_data,
            isf_err: error,
            widget_ids: None,
            updated: false,
            audio_source,
            pass_index: 0,
            image_loader,
            vs,
            fs,
            sampler,
            isf_uniform_buffer,
            isf_inputs_uniform_buffer,
            isf_bind_group_layout,
            isf_inputs_bind_group_layout,
            isf_textures_bind_group_layout,
            isf_bind_group,
            isf_inputs_bind_group,
            isf_textures_bind_group,
            layout,
            render_pipeline,
            vertex_buffer,
            dst_format,
            dst_texture_size,
            dst_sample_count,
            texture_reshaper,
        }
    }

    /// Update the ISF pipeline.
    ///
    /// Updating the ISF pipeline does the following:
    ///
    /// - First attempts to recompile the given sequence of touched shaders, both for ISF and GLSL.
    /// - Synchronises the ISF data with the latest successfully parsed `Isf` instance. Any images
    ///   that have completed loading will be uploaded to textures.
    /// - If the number of textures has changed, recreates the texture bind group, layout and
    ///   render pipeline layout.
    /// - If any of the shaders successfully recompiled, or if the number of textures changed, the
    ///   pipeline is recreated.
    pub fn encode_update<I>(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        images_path: &Path,
        touched_shaders: I,
        num_samples: u32,
    ) where
        I: IntoIterator,
        I::Item: AsRef<Path>,
    {
        // UPDATE SHADERS
        // --------------

        // Attempt to recompile touched shaders.
        let mut shader_recompiled = false;
        let mut isf_updated = false;
        for path in touched_shaders {
            let path = path.as_ref();
            if self.vs.source.as_path() == Some(&path) {
                let (module, error) = shader::compile_shader(device, &path);
                self.vs.error = error;
                if module.is_some() {
                    shader_recompiled = true;
                    self.vs.module = module;
                }
            } else if self.fs.source.as_path() == Some(&path) {
                let (module, error) = shader::compile_isf_shader(device, &path);
                self.fs.error = error;
                if module.is_some() {
                    shader_recompiled = true;
                    self.fs.module = module;
                }
                // Update the `Isf` instance.
                let isf_res = util::read_isf_from_path(&path);
                let (new_isf, new_isf_err) = util::split_result(isf_res);
                self.isf_err = new_isf_err;
                if (self.isf.is_none() || new_isf.is_some()) && self.isf != new_isf {
                    isf_updated = true;
                    self.isf = new_isf;
                    self.isf_data.end_session(&mut self.audio_source);
                }
            }
        }

        // UPDATE ISF DATA
        // ---------------

        // We can only update the isf data if we have an isf instance to work with.
        let isf = match self.isf {
            None => return,
            Some(ref isf) => isf,
        };

        // Synchronise the ISF data.
        let textures_updated = data::sync_isf_data(
            device,
            encoder,
            isf,
            self.dst_texture_size,
            &self.image_loader,
            images_path,
            &mut self.audio_source,
            &mut self.isf_data,
            num_samples,
        );

        // rebuild input buffer if isf config updated
        if isf_updated {
            let isf_input_uniforms_bytes_vec =
                data::get_isf_input_uniforms_bytes_vec(&self.isf, &self.isf_data);
            let isf_input_uniforms_bytes = &isf_input_uniforms_bytes_vec[..];

            self.isf_inputs_uniform_buffer = None;
            self.isf_inputs_bind_group_layout = None;
            self.isf_inputs_bind_group = None;
            self.widget_ids = None;

            if !isf_input_uniforms_bytes.is_empty() {
                let uniforms_usage = wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST;
                let uniform_buffer =
                    device.create_buffer_with_data(&isf_input_uniforms_bytes, uniforms_usage);
                let bind_group_layout = build_uniform_bind_group_layout(device);
                let bind_group = wgpu::BindGroupBuilder::new()
                    .buffer_bytes(&uniform_buffer, 0..1)
                    .build(device, &bind_group_layout);

                self.isf_inputs_uniform_buffer = Some(uniform_buffer);
                self.isf_inputs_bind_group_layout = Some(bind_group_layout);
                self.isf_inputs_bind_group = Some(bind_group);
            }
        }

        // UPDATE TEXTURE BIND GROUP
        // -------------------------

        // If the number of textures have changed, update the bind group and pipeline layout.
        if textures_updated || self.updated || isf_updated {
            self.isf_textures_bind_group_layout =
                create_isf_textures_bind_group_layout(device, &self.isf_data);
            self.isf_textures_bind_group = create_isf_textures_bind_group(
                device,
                &self.isf_textures_bind_group_layout,
                &self.sampler,
                &self.isf_data,
            );
        }

        if isf_updated || textures_updated || self.updated {
            let mut bind_group_layouts = vec![
                &self.isf_bind_group_layout,
                &self.isf_textures_bind_group_layout,
            ];

            if let Some(ref isf_inputs_bind_group_layout) = self.isf_inputs_bind_group_layout {
                bind_group_layouts.push(isf_inputs_bind_group_layout);
            }

            self.layout = create_pipeline_layout(device, &bind_group_layouts);
        }

        // UPDATE RENDER PIPELINE
        // ----------------------

        if shader_recompiled || textures_updated || isf_updated || self.updated {
            if let (Some(vs_mod), Some(fs_mod)) = (self.vs.module.as_ref(), self.fs.module.as_ref())
            {
                self.render_pipeline = Some(create_render_pipeline(
                    device,
                    &self.layout,
                    vs_mod,
                    fs_mod,
                    self.dst_format,
                    self.dst_sample_count,
                ));
            }
        }

        self.texture_reshaper = match self.isf_data.get_final_texture() {
            Some(texture) => Some(crate::util::create_texture_reshaper(
                device,
                &texture,
                num_samples,
            )),
            None => None,
        };

        self.updated = false;
    }

    /// Given an encoder, submits a render pass command for drawing the pipeline to the given
    /// texture.
    ///
    /// If the pipeline has not yet been created because it has not yet compiled the necessary
    /// shaders correctly, the render pass will not be encoded.
    pub fn encode_render_pass(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        dst_texture: &wgpu::TextureViewHandle,
        isf_time: IsfTime,
    ) {
        if let Some(ref pipeline) = self.render_pipeline {
            // Encode an update for the ISF uniform buffer.
            let [w, h] = self.dst_texture_size;
            let isf_uniforms = data::IsfUniforms {
                date: isf_time.date,
                render_size: [w as f32, h as f32],
                pass_index: self.pass_index as i32,
                time: isf_time.time,
                time_delta: isf_time.time_delta,
                frame_index: isf_time.frame_index,
            };
            let isf_uniforms_bytes = isf_uniforms_as_bytes(&isf_uniforms);
            let usage = wgpu::BufferUsage::COPY_SRC;
            let new_buffer = device.create_buffer_with_data(&isf_uniforms_bytes, usage);
            let uniforms_size = isf_uniforms_bytes.len() as wgpu::BufferAddress;
            encoder.copy_buffer_to_buffer(
                &new_buffer,
                0,
                &self.isf_uniform_buffer,
                0,
                uniforms_size,
            );

            // Update the input uniforms
            if let Some(ref uniform_buffer) = self.isf_inputs_uniform_buffer {
                let isf_input_uniforms_bytes_vec =
                    data::get_isf_input_uniforms_bytes_vec(&self.isf, &self.isf_data);
                let isf_input_uniforms_bytes = &isf_input_uniforms_bytes_vec[..];
                let new_buffer = device.create_buffer_with_data(&isf_input_uniforms_bytes, usage);
                let inputs_size = isf_input_uniforms_bytes.len() as wgpu::BufferAddress;
                encoder.copy_buffer_to_buffer(&new_buffer, 0, &uniform_buffer, 0, inputs_size);
            }

            // Encode the render pass.
            let mut render_pass = wgpu::RenderPassBuilder::new()
                .color_attachment(dst_texture, |color| color)
                .begin(encoder);
            render_pass.set_pipeline(pipeline);
            render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
            render_pass.set_bind_group(0, &self.isf_bind_group, &[]);
            render_pass.set_bind_group(1, &self.isf_textures_bind_group, &[]);
            if let Some(ref bind_group) = self.isf_inputs_bind_group {
                render_pass.set_bind_group(2, &bind_group, &[]);
            }

            let vertex_range = 0..VERTICES.len() as u32;
            let instance_range = 0..1;
            render_pass.draw(vertex_range, instance_range);
        }
    }

    /// Returns the current compilation error for the vertex shader if there is one.
    ///
    /// Returns `Some` if the last call to `update_shaders` contained a compilation error for the
    /// vertex shader.
    pub fn vs_err(&self) -> Option<&shader::ShaderError> {
        self.vs.error.as_ref()
    }

    /// Returns the current compilation error for the vertex shader if there is one.
    ///
    /// Returns `Some` if the last call to `update_shaders` contained a compilation error for the
    /// vertex shader.
    pub fn fs_err(&self) -> Option<&shader::ShaderError> {
        self.fs.error.as_ref()
    }

    pub fn get_program_errors(&self) -> Option<HashMap<String, String>> {
        let mut errors = HashMap::new();

        if let Some(vs_error) = self.vs_err() {
            errors.insert(String::from("Vertex Shader"), vs_error.to_string());
        }

        if let Some(fs_error) = self.fs_err() {
            errors.insert(String::from("Fragment Shader"), fs_error.to_string());
        }

        match errors.len() {
            0 => None,
            _ => Some(errors),
        }
    }

    pub fn get_audio_error(&self) -> Option<String> {
        self.audio_source.error.clone()
    }

    pub fn get_data_errors(&self) -> HashMap<String, Vec<String>> {
        let mut errors = self.isf_data.get_errors();

        if let Some(error) = self.get_audio_error() {
            errors.insert(String::from("Audio"), vec![error]);
        }

        errors
    }

    /// Generates the widget ids needed for the ISF's inputs.
    pub fn generate_widget_ids(&mut self, ui: &mut Ui) {
        let isf = match &self.isf {
            Some(i) => i,
            None => return,
        };

        let mut widget_ids = HashMap::new();

        for input in &isf.inputs {
            let name = input.name.clone();

            match &input.ty {
                isf::InputType::Float(_) => {
                    widget_ids.insert(name, ui.generate_widget_id());
                }
                isf::InputType::Long { .. } | isf::InputType::Image { .. } => {
                    widget_ids.insert(name.clone() + "-label", ui.generate_widget_id());
                    widget_ids.insert(name, ui.generate_widget_id());
                }
                isf::InputType::Point2d(_) => {
                    widget_ids.insert(name.clone() + "-label", ui.generate_widget_id());
                    widget_ids.insert(name.clone() + "-x", ui.generate_widget_id());
                    widget_ids.insert(name.clone() + "-y", ui.generate_widget_id());
                }
                isf::InputType::Color(_) => {
                    widget_ids.insert(name.clone() + "-label", ui.generate_widget_id());
                    widget_ids.insert(name.clone() + "-r", ui.generate_widget_id());
                    widget_ids.insert(name.clone() + "-g", ui.generate_widget_id());
                    widget_ids.insert(name.clone() + "-b", ui.generate_widget_id());
                    widget_ids.insert(name.clone() + "-a", ui.generate_widget_id());
                }
                _ => (),
            };
        }

        self.widget_ids = Some(widget_ids);
    }

    pub fn end_session(&mut self) {
        self.isf_data.end_session(&mut self.audio_source);
    }

    pub fn pause(&mut self) {
        self.isf_data.pause(&mut self.audio_source);
    }

    pub fn unpause(&mut self) {
        self.isf_data.unpause(&mut self.audio_source);
    }

    pub fn get_render_texture(&self, index: usize) -> &wgpu::Texture {
        self.isf_data.get_render_texture(index)
    }

    pub fn get_texture_reshaper(&self) -> Option<&wgpu::TextureReshaper> {
        self.texture_reshaper.as_ref()
    }
}
