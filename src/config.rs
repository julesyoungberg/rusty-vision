pub static SIZE: u32 = 1024;

pub static SHADERS_PATH: &str = "./src/shaders/";

pub const SHADERS: &'static [&'static str] =
    &["basic.vert", "basic.frag", "basic2.frag", "mandelbox.frag"];

pub const PIPELINES: &'static [&'static [&'static str]] = &[
    &["basic", "basic.vert", "basic.frag"],
    &["basic2", "basic.vert", "basic2.frag"],
    &["mandelbox", "basic.vert", "mandelbox.frag"],
];

pub const PROGRAMS: &'static [&'static str] = &["basic", "basic2", "mandelbox"];

pub const COLOR_MODES: &'static [&'static str] = &["palette", "solid"];
