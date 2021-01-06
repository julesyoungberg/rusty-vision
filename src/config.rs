pub static SIZE: [u32; 2] = [1920, 1080];

pub static SHADERS_PATH: &str = "./src/shaders/";

pub const COLOR_MODES: &'static [&'static str] = &["palette", "solid"];

/**
 * All shaders to compile
 */
pub const SHADERS: &'static [&'static str] = &[
    "basic.vert",
    "basic.frag",
    "mandelbox.frag",
    "mandelbulb.frag",
    "tetrahedron.frag",
];

/**
 * Shader pipeline descriptions
 * [internal_name, vertex_shader, frag_shader]
 * all shaders must be present in SHADERS
 */
pub const PIPELINES: &'static [&'static [&'static str]] = &[
    &["basic", "basic.vert", "basic.frag"],
    &["mandelbox", "basic.vert", "mandelbox.frag"],
    &["mandelbulb", "basic.vert", "mandelbulb.frag"],
    &["tetrahedron", "basic.vert", "tetrahedron.frag"],
];

/**
 * Program names, displayed to the user.
 * Must correspond with PIPELINES by index, name is irrellivant
 */
pub const PROGRAMS: &'static [&'static str] = &["basic", "mandelbox", "mandelbulb", "tetrahedron"];

pub const DEFAULT_PROGRAM: usize = 1;

/**
 * Program defaults.
 * [cam_pos, cam_target, cam_up]
 * Must correspond with PIPELINES and PROGRAMS by index
 */
pub const PROGRAM_DEFAULTS: &'static [&'static [&'static [f32; 3]]] = &[
    &[
        &[25.0, 0.0, 15.0], // cam pos
        &[0.0, 0.0, 0.0],   // cam target
        &[0.0, 1.0, 0.0],   // cam up
    ],
    &[
        &[25.0, 0.0, 15.0], // cam pos
        &[0.0, 0.0, 0.0],   // cam target
        &[0.0, 1.0, 0.0],   // cam up
    ],
    &[
        &[25.0, 0.0, 15.0], // cam pos
        &[0.0, 0.0, 0.0],   // cam target
        &[0.0, 1.0, 0.0],   // cam up
    ],
    &[
        &[3.0, 0.0, 3.0], // cam pos
        &[0.0, 0.0, 0.0], // cam target
        &[0.0, 1.0, 0.0], // cam up
    ],
];
