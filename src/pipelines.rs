#![allow(dead_code)]

use nannou::prelude::*;
use std::collections::HashMap;

#[path = "util.rs"]
mod util;

#[path = "shaders.rs"]
mod shaders;

pub type Pipelines = HashMap<String, wgpu::RenderPipeline>;

pub fn create_pipeline(
    device: &wgpu::Device,
    num_samples: u32,
    shaders: &shaders::Shaders,
    vert_name: &str,
    frag_name: &str,
) -> wgpu::RenderPipeline {
    let vert_shader = shaders::get_shader(shaders, vert_name);
    let frag_shader = shaders::get_shader(shaders, frag_name);
    let render_pipeline = util::create_pipeline(device, vert_shader, frag_shader, num_samples);
    return render_pipeline;
}

pub fn create_pipelines<'a>(
    device: &wgpu::Device,
    num_samples: u32,
    shaders: &shaders::Shaders,
    pipelines_desc: &'a [&[&str]],
) -> Pipelines {
    let mut pipelines = HashMap::new();

    for pipeline_desc in pipelines_desc {
        let name = String::from(pipeline_desc[0]);
        pipelines.insert(
            name,
            create_pipeline(
                device,
                num_samples,
                shaders,
                pipeline_desc[1],
                pipeline_desc[2],
            ),
        );
    }

    return pipelines;
}

pub fn get_pipeline<'a>(pipelines: &'a Pipelines, name: &str) -> &'a wgpu::RenderPipeline {
    match pipelines.get(name) {
        Some(pipeline) => return pipeline,
        None => {
            let mut error = "Pipeline not found: ".to_owned();
            error.push_str(name);
            panic!(error);
        }
    }
}
