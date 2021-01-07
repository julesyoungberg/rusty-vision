#![allow(dead_code)]

use nannou::prelude::*;
use std::collections::HashMap;

use crate::util;

pub type Pipelines = HashMap<String, wgpu::RenderPipeline>;

pub fn create_pipeline(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
    num_samples: u32,
    shaders: &HashMap<&String, &wgpu::ShaderModule>,
    vert_name: &str,
    frag_name: &str,
) -> wgpu::RenderPipeline {
    let vert_shader = shaders.get(&vert_name.to_string()).unwrap();
    let frag_shader = shaders.get(&frag_name.to_string()).unwrap();
    util::create_pipeline(
        device,
        bind_group_layout,
        vert_shader,
        frag_shader,
        num_samples,
    )
}

pub fn create_pipelines<'a>(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
    num_samples: u32,
    shaders: &HashMap<&String, &wgpu::ShaderModule>,
    pipelines_desc: &'a [&[&str]],
) -> Pipelines {
    let mut pipelines = HashMap::new();

    for pipeline_desc in pipelines_desc {
        let name = String::from(pipeline_desc[0]);
        pipelines.insert(
            name,
            create_pipeline(
                device,
                bind_group_layout,
                num_samples,
                shaders,
                pipeline_desc[1],
                pipeline_desc[2],
            ),
        );
    }

    pipelines
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
