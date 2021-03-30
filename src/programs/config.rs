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
pub struct ProgramSettings {
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
    pub passes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramConfig {
    pub pipeline: PipelineConfig,
    pub uniforms: Vec<String>,
    pub config: Option<ProgramSettings>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderConfig {
    pub default: String,
    pub programs: HashMap<String, ProgramConfig>,
}

impl FolderConfig {
    pub fn get_program_names(&self) -> Vec<String> {
        let mut program_names = vec![];
        for (name, _) in self.programs.iter() {
            program_names.push(name.clone());
        }
        program_names.sort();
        program_names
    }

    pub fn get_default_program_index(&self, program_names: &Vec<String>) -> Result<usize, String> {
        match program_names.iter().position(|n| *n == self.default) {
            Some(i) => Ok(i),
            None => Err(format!("Invalid default program '{}'", self.default)),
        }
    }
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

pub fn get_config(app: &App) -> Result<Config, String> {
    let root_path = util::shaders_path(app)
        .join("index.json")
        .into_os_string()
        .into_string()
        .unwrap();

    let root_json_string = match fs::read_to_string(root_path.clone()) {
        Ok(s) => s,
        Err(e) => return Err(format!("Reading {}: {}", root_path, e.to_string())),
    };

    let root_config: RootConfig = match serde_json::from_str(root_json_string.as_str()) {
        Ok(c) => c,
        Err(e) => return Err(format!("Parsing {}: {}", root_path, e.to_string())),
    };

    let mut config = Config {
        default: root_config.default,
        folders: HashMap::new(),
    };

    for folder in root_config.folders.iter() {
        let path = util::shaders_path(app)
            .join(folder)
            .join("index.json")
            .into_os_string()
            .into_string()
            .unwrap();

        let json_string = match fs::read_to_string(path.clone()) {
            Ok(s) => s,
            Err(e) => return Err(format!("Reading {}: {}", path, e.to_string())),
        };

        let folder_config: FolderConfig = match serde_json::from_str(json_string.as_str()) {
            Ok(c) => c,
            Err(e) => return Err(format!("Parsing {}: {}", path, e.to_string())),
        };

        config.folders.insert(folder.clone(), folder_config);
    }

    Ok(config)
}

impl Config {
    pub fn get_folder_names(&self) -> Vec<String> {
        let mut folder_names = vec![];
        for (name, _) in self.folders.iter() {
            folder_names.push(name.clone());
        }
        folder_names.sort();
        folder_names
    }

    pub fn get_default_folder_index(&self, folder_names: &Vec<String>) -> Result<usize, String> {
        match folder_names.iter().position(|n| *n == self.default) {
            Some(i) => Ok(i),
            None => Err(format!("Invalid default folder '{}'", self.default)),
        }
    }
}
