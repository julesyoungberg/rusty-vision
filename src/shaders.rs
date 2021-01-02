use nannou::prelude::*;
use regex::Regex;
use shaderc;
use std::fs;

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
            println!("imported: {}", import);
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
    println!("{}", complete_src);

    // compile shader
    let spirv = compiler
        .compile_into_spirv(src, kind, filename, "main", None)
        .unwrap();

    return wgpu::shader_from_spirv_bytes(device, &spirv.as_binary_u8());
}
