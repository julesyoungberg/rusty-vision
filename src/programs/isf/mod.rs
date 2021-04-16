// a fork of https://github.com/nannou-org/nannou/blob/master/nannou_isf/src/pipeline.rs

use isf;
use nannou::prelude::*;
use nannou::wgpu::BufferInitDescriptor;
use std::path::{Path, PathBuf};
use threadpool::ThreadPool;

mod data;
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
    isf: Option<isf::Isf>,
    pub isf_data: data::IsfData,
    isf_err: Option<util::IsfError>,
    image_loader: data::ImageLoader,
    vs: shader::Shader,
    fs: shader::Shader,
    sampler: wgpu::Sampler,
    isf_uniform_buffer: wgpu::Buffer,
    isf_inputs_uniform_buffer: wgpu::Buffer,
    isf_bind_group_layout: wgpu::BindGroupLayout,
    isf_inputs_bind_group_layout: wgpu::BindGroupLayout,
    isf_textures_bind_group_layout: wgpu::BindGroupLayout,
    isf_bind_group: wgpu::BindGroup,
    isf_inputs_bind_group: wgpu::BindGroup,
    isf_textures_bind_group: wgpu::BindGroup,
    layout: wgpu::PipelineLayout,
    render_pipeline: Option<wgpu::RenderPipeline>,
    vertex_buffer: wgpu::Buffer,
    dst_format: wgpu::TextureFormat,
    dst_texture_size: [u32; 2],
    dst_sample_count: u32,
}

fn isf_uniforms_as_bytes(data: &data::IsfUniforms) -> &[u8] {
    unsafe { wgpu::bytes::from(data) }
}

fn isf_input_uniforms_as_bytes(data: &[u32]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
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
    let desc = wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts,
        push_constant_ranges: &[],
    };
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
    ) -> Self {
        let isf_res = util::read_isf_from_path(&fs_path);
        let (isf, isf_error) = util::split_result(isf_res);

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
                &mut isf_data,
            );
        }

        // Prepare the uniform buffers.
        let [dst_tex_w, dst_tex_h] = dst_texture_size;
        let isf_uniforms = data::IsfUniforms {
            pass_index: 0,
            render_size: [dst_tex_w as f32, dst_tex_h as f32],
            time: 0.0,
            time_delta: 0.0,
            date: [0.0; 4],
            frame_index: 0,
        };
        let isf_uniforms_bytes = isf_uniforms_as_bytes(&isf_uniforms);
        let isf_input_uniforms: data::IsfInputUniforms = [0u32; 128];
        let isf_input_uniforms_bytes = isf_input_uniforms_as_bytes(&isf_input_uniforms);
        let uniforms_usage = wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST;
        let isf_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &isf_uniforms_bytes,
            usage: uniforms_usage,
        });
        let isf_inputs_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: &isf_input_uniforms_bytes,
            usage: uniforms_usage,
        });

        // Prepare the bind group layouts.
        let isf_bind_group_layout = wgpu::BindGroupLayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStage::FRAGMENT, false)
            .build(device);
        let isf_inputs_bind_group_layout = wgpu::BindGroupLayoutBuilder::new()
            .uniform_buffer(wgpu::ShaderStage::FRAGMENT, false)
            .build(device);
        let isf_textures_bind_group_layout =
            create_isf_textures_bind_group_layout(device, &isf_data);

        // Create the sampler.
        let sampler = wgpu::SamplerBuilder::new().build(device);

        // Create the bind groups
        let isf_bind_group = wgpu::BindGroupBuilder::new()
            .buffer::<data::IsfUniforms>(&isf_uniform_buffer, 0..1)
            .build(device, &isf_bind_group_layout);
        let isf_inputs_bind_group = wgpu::BindGroupBuilder::new()
            .buffer::<data::IsfInputUniforms>(&isf_inputs_uniform_buffer, 0..1)
            .build(device, &isf_inputs_bind_group_layout);
        let isf_textures_bind_group = create_isf_textures_bind_group(
            device,
            &isf_textures_bind_group_layout,
            &sampler,
            &isf_data,
        );

        // Create the render pipeline.
        let layout = create_pipeline_layout(
            device,
            &[
                &isf_bind_group_layout,
                &isf_inputs_bind_group_layout,
                &isf_textures_bind_group_layout,
            ],
        );
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
        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: vertices_bytes,
            usage: vertex_usage,
        });

        Self {
            isf,
            isf_data,
            isf_err,
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
    ) where
        I: IntoIterator,
        I::Item: AsRef<Path>,
    {
        // UPDATE SHADERS
        // --------------

        // Attempt to recompile touched shaders.
        let mut shader_recompiled = false;
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
                if self.isf.is_none() {
                    self.isf = new_isf;
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

        // Keep track of whether the number of textures change for our bind groups.
        let texture_count = data::isf_data_textures(&self.isf_data).count();

        // Synchronise the ISF data.
        data::sync_isf_data(
            device,
            encoder,
            isf,
            self.dst_texture_size,
            &self.image_loader,
            images_path,
            &mut self.isf_data,
        );

        // UPDATE TEXTURE BIND GROUP
        // -------------------------

        // If the number of textures have changed, update the bind group and pipeline layout.
        let new_texture_count = data::isf_data_textures(&self.isf_data).count();
        let texture_count_changed = texture_count != new_texture_count;
        if texture_count_changed {
            self.isf_textures_bind_group_layout =
                create_isf_textures_bind_group_layout(device, &self.isf_data);
            self.isf_textures_bind_group = create_isf_textures_bind_group(
                device,
                &self.isf_textures_bind_group_layout,
                &self.sampler,
                &self.isf_data,
            );
            self.layout = create_pipeline_layout(
                device,
                &[
                    &self.isf_bind_group_layout,
                    &self.isf_inputs_bind_group_layout,
                    &self.isf_textures_bind_group_layout,
                ],
            );
        }

        // UPDATE RENDER PIPELINE
        // ----------------------

        if shader_recompiled || texture_count_changed {
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
                pass_index: 0,
                render_size: [w as f32, h as f32],
                time: isf_time.time,
                time_delta: isf_time.time_delta,
                date: isf_time.date,
                frame_index: isf_time.frame_index,
            };
            let isf_uniforms_bytes = isf_uniforms_as_bytes(&isf_uniforms);
            let usage = wgpu::BufferUsage::COPY_SRC;
            let new_buffer = device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: &isf_uniforms_bytes,
                usage,
            });
            let size = isf_uniforms_bytes.len() as wgpu::BufferAddress;
            encoder.copy_buffer_to_buffer(&new_buffer, 0, &self.isf_uniform_buffer, 0, size);

            // TODO: Update the inputs.
            let _ = &self.isf_inputs_uniform_buffer;
            //let size = std::mem::size_of::<IsfInputUniforms>() as wgpu::BufferAddress;
            //encoder.copy_buffer_to_buffer(&new_buffer, 0, &self.isf_inputs_uniform_buffer, 0, size);

            // Encode the render pass.
            let mut render_pass = wgpu::RenderPassBuilder::new()
                .color_attachment(dst_texture, |color| color)
                .begin(encoder);
            render_pass.set_pipeline(pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.isf_bind_group, &[]);
            render_pass.set_bind_group(1, &self.isf_inputs_bind_group, &[]);
            render_pass.set_bind_group(2, &self.isf_textures_bind_group, &[]);
            let vertex_range = 0..VERTICES.len() as u32;
            let instance_range = 0..1;
            render_pass.draw(vertex_range, instance_range);
        }
    }

    /// Encode a render pass command for drawing the output of the pipeline to the given frame.
    ///
    /// Uses `encode_render_pass` internally.
    pub fn encode_to_frame(&self, frame: &Frame, isf_time: IsfTime) {
        let device = frame.device_queue_pair().device();
        let mut encoder = frame.command_encoder();
        self.encode_render_pass(device, &mut *encoder, frame.texture_view(), isf_time);
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
}