use nannou::prelude::*;
use regex::Regex;
use std::fs;
use std::panic;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::thread;

use crate::util;

/// Stores data that represents a single shader file
/// and manages the compiling of a shader.
#[derive(Debug)]
pub struct Shader {
    pub error: Option<String>,
    pub filename: String,
    pub module: Option<wgpu::ShaderModule>,
}

impl Shader {
    pub fn new(filename: String) -> Self {
        Self {
            error: None,
            filename,
            module: None,
        }
    }

    /// Compile the shader file
    pub fn compile(
        &mut self,
        shaders_path: PathBuf,
        device: &wgpu::Device,
        compiler: &mut shaderc::Compiler,
    ) {
        let split = self.filename.split('.').collect::<Vec<&str>>();
        let ext = split[1];
        let mut kind = shaderc::ShaderKind::Fragment;
        if ext == "vert" {
            kind = shaderc::ShaderKind::Vertex;
        }

        let filename = shaders_path
            .join(self.filename.clone())
            .into_os_string()
            .into_string()
            .unwrap();
        println!("reading: {}", filename);
        let src_string = fs::read_to_string(util::universal_path(filename.clone()))
            .unwrap_or_else(|_| panic!("Error reading shader: {}", filename));

        let (tx, rx) = channel();

        // load shader dependencies ([^\r]*) deals with \r on windows
        let re = Regex::new(r"//@import ([^\r\n]*)").unwrap();
        let flnm = filename.clone();
        thread::spawn(move || {
            let complete = re
                .replace_all(src_string.as_str(), |captures: &regex::Captures| {
                    let import = &captures[1];
                    let import_path = match shaders_path
                        .clone()
                        .join(format!("{}.glsl", import))
                        .into_os_string()
                        .into_string()
                    {
                        Ok(s) => s,
                        Err(_) => {
                            tx.send(Err(format!(
                                "Error building path for {} from {}",
                                import, flnm
                            )))
                            .unwrap();
                            return "".to_string();
                        }
                    };

                    let mut import_src = "\n".to_owned();
                    let import_src_string = match fs::read_to_string(import_path.clone()) {
                        Ok(s) => s,
                        Err(_) => {
                            tx.send(Err(format!(
                                "Error importing {} from {}",
                                import_path, flnm
                            )))
                            .unwrap();
                            return "".to_string();
                        }
                    };
                    import_src.push_str(import_src_string.as_str());
                    import_src
                })
                .to_string();
            tx.send(Ok(complete)).unwrap();
        });

        let complete_src = match rx.recv().unwrap() {
            Ok(s) => s,
            Err(err) => {
                self.error = Some(err);
                return;
            }
        };

        // compile shader
        match compiler.compile_into_spirv(
            complete_src.as_str(),
            kind,
            filename.as_str(),
            "main",
            None,
        ) {
            Ok(program) => {
                self.module = Some(wgpu::shader_from_spirv_bytes(
                    device,
                    &program.as_binary_u8(),
                ));
                self.error = None;
            }
            Err(e) => {
                self.error = Some(e.to_string());
            }
        }
    }
}
