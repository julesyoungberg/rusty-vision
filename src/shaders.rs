use nannou::prelude::*;
use shaderc;
use std::fs;

static SHADERS_PATH: &str = "./src/shaders/";

pub fn compile_shader(
    device: &wgpu::Device,
    compiler: &mut shaderc::Compiler,
    filename: &str,
    kind: shaderc::ShaderKind,
) -> wgpu::ShaderModule {
    // create error message
    let mut error = "Error reading shader: ".to_owned();
    error.push_str(filename);

    // build path
    let mut path = SHADERS_PATH.to_owned();
    path.push_str(filename);

    // read and compile shader
    let src_string = fs::read_to_string(path).expect(error.as_str());
    let src = src_string.as_str();
    let spirv = compiler
        .compile_into_spirv(src, kind, filename, "main", None)
        .unwrap();

    return wgpu::shader_from_spirv_bytes(device, &spirv.as_binary_u8());
}
