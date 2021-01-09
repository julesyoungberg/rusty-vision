pub static SIZE: [u32; 2] = [1920, 1080];

pub static SHADERS_PATH: &str = "./src/shaders/";

pub const COLOR_MODES: &'static [&'static str] = &["palette", "solid"];

/**
 * Shader pipeline descriptions
 * [internal_name, vertex_shader, frag_shader]
 */
pub const PIPELINES: &'static [&'static [&'static str]] = &[
    &["basic", "basic.vert", "basic.frag"],
    &["mandelbox", "basic.vert", "mandelbox.frag"],
    &["mandelbulb", "basic.vert", "mandelbulb.frag"],
    &["tetrahedron", "basic.vert", "tetrahedron.frag"],
];

/**
 * Program names, displayed to the user.
 * Must correspond with PIPELINES by name
 */
pub const PROGRAMS: &'static [&'static str] = &["basic", "mandelbox", "mandelbulb", "tetrahedron"];

pub const DEFAULT_PROGRAM: usize = 1;

/**
 * Program defaults.
 * [cam_pos, cam_target, cam_up, shape_rotation, [color_mode, ?, ?]]
 * Must correspond with PROGRAMS by index
 */
pub const PROGRAM_DEFAULTS: &'static [&'static [&'static [f32; 3]]] = &[
    &[
        // basic
        &[25.0, 0.0, 15.0], // cam pos
        &[0.0, 0.0, 0.0],   // cam target
        &[0.0, 1.0, 0.0],   // cam up
        &[0.0, 0.0, 0.0],   // shape rotation
        &[0.0, 0.0, 0.0],   // [color_mode, ?, ?]
    ],
    &[
        // mandelbox
        &[25.0, 0.0, 15.0], // cam pos
        &[0.0, 0.0, 0.0],   // cam target
        &[0.0, 1.0, 0.0],   // cam up
        &[0.0, 0.0, 0.0],   // shape rotation
        &[0.0, 0.0, 0.0],   // [color_mode, ?, ?]
    ],
    &[
        // mandelbulb
        &[5.0, 0.0, 5.0], // cam pos
        &[0.0, 0.0, 0.0], // cam target
        &[0.0, 1.0, 0.0], // cam up
        &[0.0, 0.0, 0.0], // shape rotation
        &[0.0, 0.0, 0.0], // [color_mode, ?, ?]
    ],
    &[
        // tetrahedron
        &[7.0, 0.68, 6.72],   // cam pos
        &[2.0, 0.48, 1.72],   // cam target
        &[-0.02, 1.0, -0.02], // cam up
        &[35.0, 0.0, 315.0],  // shape rotation
        &[1.0, 0.0, 0.0],     // [color_mode, ?, ?]
    ],
];

/**
 * A list of uniform buffer lists for each program.
 * Must correspond with PROGRAMS by index
 */
pub const PROGRAM_UNIFORMS: &'static [&'static str] = &[
    "general",          // basic
    "general,geometry", // mandelbox
    "general,geometry", // mandelbulb
    "general,geometry", // tetrahedron
];
