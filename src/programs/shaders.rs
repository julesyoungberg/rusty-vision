use nannou::prelude::*;
use regex::Regex;
use shaderc;
use std::collections::HashMap;
use std::fs;

use crate::config;

/**
 * Stores data that represents a single shader file
 */
#[derive(Debug)]
pub struct Shader {
    pub error: Option<shaderc::Error>,
    pub filename: String,
    pub module: Option<wgpu::ShaderModule>,
}

/**
 * Manages the compiling of a shader
 */
impl Shader {
    pub fn new(filename: String) -> Self {
        Self {
            error: None,
            filename,
            module: None,
        }
    }

    /**
     * Compile the shader file
     */
    pub fn compile(&mut self, device: &wgpu::Device, compiler: &mut shaderc::Compiler) {
        let split = self.filename.split(".").collect::<Vec<&str>>();
        let ext = split[1];
        let mut kind = shaderc::ShaderKind::Fragment;
        if ext == "vert" {
            kind = shaderc::ShaderKind::Vertex;
        }

        let filename = self.filename.as_str();

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
                let import_src_string =
                    fs::read_to_string(import_path).expect(import_error.as_str());
                import_src.push_str(import_src_string.as_str());
                return import_src;
            })
            .to_string();

        // compile shader
        match compiler.compile_into_spirv(complete_src.as_str(), kind, filename, "main", None) {
            Ok(program) => {
                self.module = Some(wgpu::shader_from_spirv_bytes(
                    device,
                    &program.as_binary_u8(),
                ));
                self.error = None;
            }
            Err(e) => {
                self.error = Some(e);
                self.module = None;
            }
        }
    }
}

pub type Shaders = HashMap<String, Shader>;

/**
 * Stores a collection of shaders
 */
#[derive(Debug)]
pub struct ShaderStore {
    pub shaders: Shaders,
}

/**
 * Manages the compiling of a collection of shaders
 */
impl ShaderStore {
    pub fn new() -> Self {
        let mut shaders = HashMap::new();

        for name in config::SHADERS {
            shaders.insert(name.to_string(), Shader::new(name.to_string()));
        }

        Self { shaders }
    }

    // TODO: parallelize
    pub fn compile(&mut self, device: &wgpu::Device) {
        let mut compiler = shaderc::Compiler::new().unwrap();
        for (_, shader) in self.shaders.iter_mut() {
            shader.compile(device, &mut compiler);
        }
    }

    pub fn errors(&self) -> HashMap<&String, &shaderc::Error> {
        let mut e = HashMap::new();
        for (name, shader) in self.shaders.iter() {
            if let Some(error) = &shader.error {
                e.insert(name, error);
            }
        }
        e
    }
}
