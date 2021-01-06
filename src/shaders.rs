#![allow(dead_code)]

use nannou::prelude::*;
use regex::Regex;
use shaderc;
use std::collections::HashMap;
use std::fs;

use crate::config;

pub type Shaders = HashMap<String, wgpu::ShaderModule>;

pub type CompilationErrors = HashMap<String, shaderc::Error>;

pub struct CompilationResult {
    pub errors: CompilationErrors,
    pub shaders: Shaders,
}

pub fn compile_shader(
    device: &wgpu::Device,
    compiler: &mut shaderc::Compiler,
    filename: &str,
) -> Result<wgpu::ShaderModule, shaderc::Error> {
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
    let mut path = config::SHADERS_PATH.to_owned();
    path.push_str(filename);

    // read shader
    let src_string = fs::read_to_string(path).expect(error.as_str());
    let src = src_string.as_str();

    // load shader dependencies
    let re = Regex::new(r"\n//@import (.*)").unwrap();
    let complete_src = re
        .replace_all(src, |captures: &regex::Captures| {
            let import = &captures[1];
            let mut import_path = config::SHADERS_PATH.to_owned();
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
    match compiler.compile_into_spirv(complete_src.as_str(), kind, filename, "main", None) {
        Ok(program) => Ok(wgpu::shader_from_spirv_bytes(
            device,
            &program.as_binary_u8(),
        )),
        Err(e) => Err(e),
    }
}

pub fn compile_shaders(device: &wgpu::Device, shader_names: &[&str]) -> CompilationResult {
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut shaders = HashMap::new();
    let mut errors = HashMap::new();

    for shader in shader_names {
        let key = shader.to_string();
        match compile_shader(device, &mut compiler, shader) {
            Ok(p) => {
                shaders.insert(key, p);
            }
            Err(e) => {
                println!("{}", e);
                errors.insert(key, e);
            }
        }
    }

    CompilationResult { errors, shaders }
}

pub fn get_shader<'a>(shaders: &'a Shaders, filename: &str) -> &'a wgpu::ShaderModule {
    match shaders.get(filename) {
        Some(shader) => return shader,
        None => {
            let mut error = "Shader not found: ".to_owned();
            error.push_str(filename);
            panic!(error);
        }
    }
}
