use nannou::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::util;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineConfig {
    pub vert: Option<String>,
    pub frag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramDefaults {
    pub audio_feature_smoothing: Option<f32>,
    pub audio_fft_smoothing: Option<f32>,
    pub camera_position: Option<Vector3<f32>>,
    pub camera_target: Option<Vector3<f32>>,
    pub camera_up: Option<Vector3<f32>>,
    pub color_mode: Option<u32>,
    pub shape_rotation: Option<Vector3<f32>>,
    pub image1: Option<String>,
    pub image2: Option<String>,
    pub noise_lacunarity: Option<f32>,
    pub noise_gain: Option<f32>,
    pub noise_invert: Option<i32>,
    pub noise_mirror: Option<i32>,
    pub noise_octaves: Option<i32>,
    pub noise_scale_by_prev: Option<i32>,
    pub noise_sharpen: Option<i32>,
    pub noise_speed: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramConfig {
    pub pipeline: PipelineConfig,
    pub uniforms: Vec<String>,
    pub defaults: Option<ProgramDefaults>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderConfig {
    pub default: String,
    pub programs: HashMap<String, ProgramConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootConfig {
    pub default: String,
    pub folders: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub default: String,
    pub folders: HashMap<String, FolderConfig>,
}

pub fn get_config(app: &App) -> Config {
    let root_path = util::shaders_path(app)
        .join("index.json")
        .into_os_string()
        .into_string()
        .unwrap();
    let root_json_string = fs::read_to_string(root_path.clone())
        .unwrap_or_else(|_| panic!("Error reading '{:?}'", root_path));
    let root_config: RootConfig = serde_json::from_str(root_json_string.as_str())
        .unwrap_or_else(|_| panic!("Error parsing '{:?}'", root_path));

    let mut config = Config {
        default: root_config.default,
        folders: HashMap::new(),
    };

    root_config.folders.iter().for_each(|folder| {
        let path = util::shaders_path(app)
            .join(folder)
            .join("index.json")
            .into_os_string()
            .into_string()
            .unwrap();

        let json_string = match fs::read_to_string(path.clone()) {
            Ok(s) => s,
            Err(_) => return,
        };
        let folder_config: FolderConfig = match serde_json::from_str(json_string.as_str()) {
            Ok(c) => c,
            Err(_) => return,
        };

        config.folders.insert(folder.clone(), folder_config);
    });

    config
}
