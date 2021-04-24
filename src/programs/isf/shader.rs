// A fork of https://github.com/nannou-org/nannou/blob/master/nannou_isf/src/pipeline.rs

use nannou::prelude::*;
use std::path::{Path, PathBuf};
use thiserror::Error;

use crate::programs::isf::util;

/// Errors that might occur while loading a shader.
#[derive(Debug, Error)]
pub enum ShaderError {
    #[error("{err}")]
    Io {
        #[from]
        err: std::io::Error,
    },
    #[error("an error occurred while parsing ISF: {err}")]
    IsfParse {
        #[from]
        err: isf::ParseError,
    },
    #[error("an error occurred while parsing ISF: {err}")]
    Compile {
        #[from]
        err: hotglsl::CompileError,
    },
}

// Check whether or not any of the given list of isf inputs would require the `IsfDataInputs`
// uniform.
fn inputs_require_isf_data_input(inputs: &[isf::Input]) -> bool {
    for input in inputs {
        match input.ty {
            isf::InputType::Image | isf::InputType::Audio(_) | isf::InputType::AudioFft(_) => (),
            _ => return true,
        }
    }
    false
}

/// Generate the necessary GLSL declarations from the given ISF to be prefixed to the GLSL string
/// from which the ISF was parsed.
///
/// This string should be inserted directly after the version preprocessor.
pub fn glsl_string_from_isf(isf: &isf::Isf) -> String {
    // The normalised coords passed through from the vertex shader.
    let frag_norm_coord_str = "
        layout(location = 0) in vec2 isf_FragNormCoord;
    ";

    // Create the `IsfData` uniform buffer with time, date, etc.
    let isf_data_str = "
        layout(set = 0, binding = 0) uniform IsfData {
            vec4 DATE;
            vec2 RENDERSIZE;
            int PASSINDEX;
            float TIME;
            float TIMEDELTA;
            int FRAMEINDEX;
        };
    ";

    // Create the `img_sampler` binding, used for sampling all input images.
    let img_sampler_str = "
        layout(set = 1, binding = 0) uniform sampler img_sampler;
    ";

    // Create the textures for the "IMPORTED" images.
    let mut binding = 1;
    let mut imported_textures = vec![];
    for name in isf.imported.keys() {
        let s = format!(
            "layout(set = 1, binding = {}) uniform texture2D {};\n",
            binding, name
        );
        imported_textures.push(s);
        binding += 1;
    }

    // Create the `texture2D` bindings for image, audio and audioFFT inputs.
    let mut input_textures = vec![];
    for input in &isf.inputs {
        match input.ty {
            isf::InputType::Image | isf::InputType::Audio(_) | isf::InputType::AudioFft(_) => {}
            _ => continue,
        }
        let s = format!(
            "layout(set = 1, binding = {}) uniform texture2D {};\n",
            binding, input.name
        );
        input_textures.push(s);
        binding += 1;
    }

    // Now create textures for the `PASSES`.
    let mut pass_textures = vec![];
    for pass in &isf.passes {
        let target = match pass.target {
            None => continue,
            Some(ref t) => t,
        };
        let s = format!(
            "layout(set = 1, binding = {}) uniform texture2D {};\n",
            binding, target
        );
        pass_textures.push(s);
        binding += 1;
    }

    // Create the `IsfDataInputs` uniform buffer with a field for each event, float, long, bool,
    // point2d and color.
    let isf_data_input_str = match inputs_require_isf_data_input(&isf.inputs) {
        false => None,
        true => {
            let mut isf_data_input_string = "
                layout(set = 2, binding = 0) uniform IsfDataInputs {\n
            "
            .to_string();
            for input in &isf.inputs {
                let ty_str = match input.ty {
                    isf::InputType::Event | isf::InputType::Bool(_) => "bool",
                    isf::InputType::Long(_) => "int",
                    isf::InputType::Float(_) => "float",
                    isf::InputType::Point2d(_) => "vec2",
                    isf::InputType::Color(_) => "vec4",
                    isf::InputType::Image
                    | isf::InputType::Audio(_)
                    | isf::InputType::AudioFft(_) => continue,
                };
                isf_data_input_string.push_str(&format!("{} {};\n", ty_str, input.name));
            }
            isf_data_input_string.push_str("};\n");
            Some(isf_data_input_string)
        }
    };

    // Image functions.
    let img_fns_str = "
        // ISF provided short-hand for retrieving image size.
        ivec2 IMG_SIZE(texture2D img) {
            return textureSize(sampler2D(img, img_sampler), 0);
        }

        // ISF provided short-hand for retrieving image color.
        vec4 IMG_NORM_PIXEL(texture2D img, vec2 norm_px_coord) {
            return texture(sampler2D(img, img_sampler), norm_px_coord);
        }

        // ISF provided short-hand for retrieving image color.
        vec4 IMG_PIXEL(texture2D img, vec2 px_coord) {
            ivec2 s = IMG_SIZE(img);
            vec2 norm_px_coord = vec2(px_coord.x / float(s.x), 1.0 - px_coord.y / float(s.y));
            return IMG_NORM_PIXEL(img, px_coord);
        }

        // ISF provided short-hand for retrieving image color.
        vec4 IMG_THIS_NORM_PIXEL(texture2D img) {
            vec2 c = vec2(isf_FragNormCoord.x, 1.0 - isf_FragNormCoord.y);
            return IMG_NORM_PIXEL(img, c);
        }

        // ISF provided short-hand for retrieving image color.
        vec4 IMG_THIS_PIXEL(texture2D img) {
            return IMG_THIS_NORM_PIXEL(img);
        }
    ";

    // Combine all the declarations together.
    let mut s = String::new();
    s.push_str(&frag_norm_coord_str);
    s.push_str(&isf_data_str);
    s.push_str(&img_sampler_str);
    s.extend(imported_textures);
    s.extend(input_textures);
    s.extend(pass_textures);
    s.extend(isf_data_input_str);
    s.push_str(&img_fns_str);
    s
}

/// Check to see if the `gl_FragColor` variable from old GLSL versions exist and if there's no out
/// variable for it. If so, create a variable for it.
///
/// TODO: This should check that `gl_FragColor` doesn't just exist in a comment or behind a
/// pre-existing macro or something. This was originally just added to makes the tests past.
pub fn glfragcolor_exists_and_no_out(glsl_str: &str) -> bool {
    glsl_str.contains("gl_FragColor") && !glsl_str.contains("out vec4 gl_FragColor")
}

/// We can't create allow a `gl_FragColor` out, so in the case we have to rename it we create the
/// out decl for it here.
pub const FRAGCOLOR_OUT_DECL_STR: &str = "layout(location = 0) out vec4 FragColor;";

/// Inserts the ISF into the beginning of the shader, returning the resulting glsl source.
pub fn prefix_isf_glsl_str(isf_glsl_str: &str, mut shader_string: String) -> String {
    // Check to see if we need to declare the `gl_FragCoord` output.
    // While we're at it, replace `vv_FragNormCoord` with `isf_FragNormCoord` if necessary.
    let glfragcolor_decl_str = {
        shader_string = shader_string.replace("vv_FragNormCoord", "isf_FragNormCoord");
        if glfragcolor_exists_and_no_out(&shader_string) {
            shader_string = shader_string.replace("gl_FragColor", "FragColor");
            Some(FRAGCOLOR_OUT_DECL_STR.to_string())
        } else {
            None
        }
    };

    // See if the version exists or if it needs to be added.
    enum Version {
        // Where the version currently exists.
        Exists(std::ops::Range<usize>),
        // The version string that needs to be added.
        NeedsToBeAdded(&'static str),
    }
    // TODO: This will break if there's a commented line like `//#version` before the actual
    // version. This caveat is possibly worth the massive speedup we gain by not parsing with
    // `glsl` crate.
    let version = if let Some(start) = shader_string.find("#version ") {
        let version_line = shader_string[start..]
            .lines()
            .next()
            .expect("failed to retrieve verison line");
        let end = start + version_line.len();
        Version::Exists(start..end)
    } else {
        Version::NeedsToBeAdded("#version 450\n")
    };

    // The output string we will fill and return.
    let mut output = String::new();

    // Add the version to the top. Grab the remaining part of the shader string yet to be added.
    let remaining_shader_str = match version {
        Version::NeedsToBeAdded(s) => {
            output.push_str(s);
            &shader_string
        }
        Version::Exists(range) => {
            output.push_str(&format!("{}\n", &shader_string[range.clone()]));
            &shader_string[range.end..]
        }
    };

    output.extend(glfragcolor_decl_str);
    output.push_str(isf_glsl_str);
    output.push_str(remaining_shader_str);
    output
}

/// Compile an ISF fragment shader.
///
/// This is used for compiling the ISF fragment shader.
pub fn compile_isf_shader(
    device: &wgpu::Device,
    path: &Path,
) -> (Option<wgpu::ShaderModule>, Option<ShaderError>) {
    let res = std::fs::read_to_string(&path)
        .map_err(ShaderError::from)
        .and_then(|s| isf::parse(&s).map(|isf| (s, isf)).map_err(From::from))
        .and_then(|(old_str, isf)| {
            let isf_str = glsl_string_from_isf(&isf);
            let new_str = prefix_isf_glsl_str(&isf_str, old_str);
            let ty = hotglsl::ShaderType::Fragment;
            hotglsl::compile_str(&new_str, ty).map_err(From::from)
        });
    let (bytes, error) = util::split_result(res);
    let module = bytes.map(|b| wgpu::shader_from_spirv_bytes(device, &b));
    (module, error)
}

/// Compile a regular, non-ISF shader.
///
/// This is used for compiling the vertex shaders.
pub fn compile_shader(
    device: &wgpu::Device,
    path: &Path,
) -> (Option<wgpu::ShaderModule>, Option<ShaderError>) {
    let res = hotglsl::compile(&path).map_err(ShaderError::from);
    let (bytes, compile_err) = util::split_result(res);
    let module = bytes.map(|b| wgpu::shader_from_spirv_bytes(device, &b));
    (module, compile_err)
}

/// Compile a regular, non-ISF shader.
///
/// This is used for compiling the vertex shaders.
pub fn compile_inline_shader(
    device: &wgpu::Device,
    code: &str,
) -> (Option<wgpu::ShaderModule>, Option<ShaderError>) {
    let res = hotglsl::compile_str(code, hotglsl::ShaderType::Vertex).map_err(From::from);
    let (bytes, compile_err) = util::split_result(res);
    let module = bytes.map(|b| wgpu::shader_from_spirv_bytes(device, &b));
    (module, compile_err)
}

#[derive(Debug)]
pub enum ShaderSource {
    Path(PathBuf),
    HardCoded,
}

impl ShaderSource {
    pub fn as_path(&self) -> Option<&Path> {
        match *self {
            ShaderSource::Path(ref path) => Some(path),
            ShaderSource::HardCoded => None,
        }
    }
}

/// A shader with some extra information relating to recent compilation success/failure.
#[derive(Debug)]
pub struct Shader {
    pub source: ShaderSource,
    pub module: Option<wgpu::ShaderModule>,
    pub error: Option<ShaderError>,
}

impl Shader {
    pub fn fragment_from_path(device: &wgpu::Device, path: PathBuf) -> Self {
        let (module, error) = compile_isf_shader(device, &path);
        let source = ShaderSource::Path(path);
        Shader {
            source,
            module,
            error,
        }
    }

    pub fn vertex_from_path(device: &wgpu::Device, path: PathBuf) -> Self {
        let (module, error) = compile_shader(device, &path);
        let source = ShaderSource::Path(path);
        Shader {
            source,
            module,
            error,
        }
    }

    /// Create the default vertex shader for ISF fragment shaders.
    pub fn vertex_default(device: &wgpu::Device) -> Self {
        let vs = include_str!("shaders/default.vs");
        let (module, error) = compile_inline_shader(device, vs);
        let source = ShaderSource::HardCoded;
        Shader {
            source,
            module,
            error,
        }
    }
}
