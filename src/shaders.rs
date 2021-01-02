use nannou::prelude::*;
use notify::{watcher, RecursiveMode, Watcher};
use regex::Regex;
use shaderc;
use std::collections::HashMap;
use std::fs;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

#[path = "util.rs"]
mod util;

static SHADERS_PATH: &str = "./src/shaders/";

pub fn compile_shader(
    device: &wgpu::Device,
    compiler: &mut shaderc::Compiler,
    filename: &str,
) -> wgpu::ShaderModule {
    let split = filename.split(".").collect::<Vec<&str>>();
    let ext = split[1];
    let mut kind = shaderc::ShaderKind::Fragment;
    if ext == "vert" {
        kind = shaderc::ShaderKind::Vertex;
    }

    // create error message
    let mut error = "Error reading shader: ".to_owned();
    error.push_str(filename);

    // build path
    let mut path = SHADERS_PATH.to_owned();
    path.push_str(filename);

    // read shader
    let src_string = fs::read_to_string(path).expect(error.as_str());
    let src = src_string.as_str();

    // load shader dependencies
    let re = Regex::new(r"\n//@import (.*)").unwrap();
    let complete_src = re
        .replace_all(src, |captures: &regex::Captures| {
            let import = &captures[1];
            let mut import_path = SHADERS_PATH.to_owned();
            import_path.push_str(import);
            import_path.push_str(".glsl");

            let mut import_error = "Error reading shader '".to_owned();
            import_error.push_str(filename);
            import_error.push_str("' import '");
            import_error.push_str(import);
            import_error.push_str("'");

            let mut import_src = "\n".to_owned();
            let import_src_string = fs::read_to_string(import_path).expect(import_error.as_str());
            import_src.push_str(import_src_string.as_str());
            return import_src;
        })
        .to_string();

    // compile shader
    let spirv = compiler
        .compile_into_spirv(complete_src.as_str(), kind, filename, "main", None)
        .unwrap();

    return wgpu::shader_from_spirv_bytes(device, &spirv.as_binary_u8());
}

pub fn compile_shaders(
    device: &wgpu::Device,
    shader_names: &[&str],
) -> HashMap<String, wgpu::ShaderModule> {
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut shaders = HashMap::new();

    for shader in shader_names {
        let key = shader.to_string();
        shaders.insert(key, compile_shader(device, &mut compiler, shader));
    }

    return shaders;
}

pub fn get_shader<'a>(
    shaders: &'a HashMap<String, wgpu::ShaderModule>,
    filename: &str,
) -> &'a wgpu::ShaderModule {
    match shaders.get(filename) {
        Some(shader) => return &shader,
        None => {
            let mut error = "Shader not found: ".to_owned();
            error.push_str(filename);
            panic!(error);
        }
    }
}

pub fn create_pipeline<'a>(
    device: &wgpu::Device,
    num_samples: u32,
    shaders: HashMap<String, wgpu::ShaderModule>,
    vert_name: &str,
    frag_name: &str,
) -> wgpu::RenderPipeline {
    let vert_shader = get_shader(&shaders, vert_name);
    let frag_shader = get_shader(&shaders, frag_name);
    let render_pipeline = util::create_pipeline(device, vert_shader, frag_shader, num_samples);
    return render_pipeline;
}

pub fn watch() {
    thread::spawn(|| {
        let (tx, rx) = channel();

        let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

        watcher
            .watch(SHADERS_PATH, RecursiveMode::Recursive)
            .unwrap();

        loop {
            match rx.recv() {
                Ok(event) => println!("{:?}", event),
                Err(e) => println!("watch error: {:?}", e),
            }
        }
    });
}
