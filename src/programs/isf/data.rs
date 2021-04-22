// a fork of https://github.com/nannou-org/nannou/blob/master/nannou_isf/src/pipeline.rs

#![allow(dead_code)]

use nannou::image;
use nannou::prelude::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use thiserror::Error;
use threadpool::ThreadPool;

pub const DEFAULT_AUDIO_SAMPLE_COUNT: u32 = 64;
pub const DEFAULT_AUDIO_FFT_COLUMNS: u32 = 64;
pub const DEFAULT_AUDIO_TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::R32Float;

/// Handles to both the cpu and gpu representations of the image.
#[derive(Debug)]
pub struct ImageData {
    pub image: image::RgbaImage,
    pub texture: wgpu::Texture,
}

#[derive(Debug)]
pub struct ImageLoader {
    pub threadpool: ThreadPool,
}

/// Errors that might occur while loading an image.
#[derive(Debug, Error)]
pub enum ImageLoadError {
    #[error("an IO error: {err}")]
    Io {
        #[from]
        err: std::io::Error,
    },
    #[error("{}", err)]
    Image {
        #[from]
        err: image::ImageError,
    },
}

pub type ImportName = String;
pub type InputName = String;

/// The state of the image.
#[derive(Debug)]
pub enum ImageState {
    None,
    Loading(mpsc::Receiver<Result<image::RgbaImage, ImageLoadError>>),
    Ready(Result<ImageData, ImageLoadError>),
}

impl ImageState {
    /// Whether or not the texture is currently loading.
    pub fn is_loading(&self) -> bool {
        matches!(*self, ImageState::Loading(_))
    }

    /// If the image has been loaded, provides access to the result.
    ///
    /// Returns `None` if the image is still loading or has not started loading.
    pub fn ready(&self) -> Option<Result<&ImageData, &ImageLoadError>> {
        match *self {
            ImageState::Ready(ref res) => Some(res.as_ref()),
            _ => None,
        }
    }

    /// Update the image state.
    fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        image_loader: &ImageLoader,
        img_path: PathBuf,
    ) {
        *self = match *self {
            ImageState::None => {
                let (tx, rx) = mpsc::channel();
                image_loader.threadpool.execute(move || {
                    let img_res = image::open(img_path)
                        .map(|img| img.to_rgba8())
                        .map_err(|err| err.into());
                    tx.send(img_res).ok();
                });
                ImageState::Loading(rx)
            }
            ImageState::Loading(ref rx) => match rx.try_recv() {
                Ok(img_res) => {
                    let usage = wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED;
                    let res = img_res.map(|image| {
                        let texture = wgpu::Texture::encode_load_from_image_buffer(
                            device, encoder, usage, &image,
                        );
                        ImageData { image, texture }
                    });
                    ImageState::Ready(res)
                }
                _ => return,
            },
            ImageState::Ready(_) => return,
        };
    }
}

#[derive(Debug)]
pub enum IsfInputData {
    Event {
        happening: bool,
    },
    Bool(bool),
    Long(i32),
    Float(f32),
    Point2d(Point2),
    Color(LinSrgba),
    Image(ImageState),
    Audio {
        samples: Vec<f32>,
        texture: wgpu::Texture,
    },
    AudioFft {
        columns: Vec<f32>,
        texture: wgpu::Texture,
    },
}

/// Given a path to a directory, produces the paths of all images within it.
fn image_paths(dir: &Path) -> impl Iterator<Item = PathBuf> {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|res| res.ok())
        .map(|entry| entry.path().to_path_buf())
        .filter(|path| image::image_dimensions(path).ok().is_some())
}

fn default_isf_texture_usage() -> wgpu::TextureUsage {
    wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED
}

fn create_black_texture(
    device: &wgpu::Device,
    encoder: &mut wgpu::CommandEncoder,
    size: [u32; 2],
    format: wgpu::TextureFormat,
) -> wgpu::Texture {
    let texture = wgpu::TextureBuilder::new()
        .usage(default_isf_texture_usage())
        .size(size)
        .format(format)
        .build(device);
    let data = vec![0u8; texture.size_bytes()];
    texture.upload_data(device, encoder, &data);
    texture
}

impl IsfInputData {
    /// Initialise a new `IsfInputData` instance.
    fn new(
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        image_loader: &ImageLoader,
        images_path: &Path,
        input: &isf::Input,
    ) -> Self {
        match &input.ty {
            isf::InputType::Event => IsfInputData::Event { happening: false },
            isf::InputType::Bool(b) => IsfInputData::Bool(b.default.unwrap_or_default()),
            isf::InputType::Long(n) => {
                let init = n
                    .default
                    .or(n.min)
                    .or_else(|| n.values.first().cloned())
                    .unwrap_or_default();
                IsfInputData::Long(init)
            }
            isf::InputType::Float(f) => {
                let init = f.default.or(f.min).unwrap_or_default();
                IsfInputData::Float(init)
            }
            isf::InputType::Point2d(p) => {
                let [x, y] = p.default.or(p.min).unwrap_or_default();
                IsfInputData::Point2d(pt2(x, y))
            }
            isf::InputType::Color(c) => {
                let v = c
                    .default
                    .clone()
                    .or_else(|| c.min.clone())
                    .unwrap_or_default();
                let r = v.get(0).cloned().unwrap_or_default();
                let g = v.get(1).cloned().unwrap_or_default();
                let b = v.get(2).cloned().unwrap_or_default();
                let a = v.get(3).cloned().unwrap_or_default();
                IsfInputData::Color(lin_srgba(r, g, b, a))
            }
            // For the input images, it's up to us how we want to source them. Perhaps
            // `assets/images/`?  For now we'll black images.
            isf::InputType::Image => {
                let mut image_state = ImageState::None;
                if let Some(img_path) = image_paths(images_path).next() {
                    image_state.update(device, encoder, image_loader, img_path);
                }
                IsfInputData::Image(image_state)
            }
            isf::InputType::Audio(a) => {
                let n_samples = a.num_samples.unwrap_or(DEFAULT_AUDIO_SAMPLE_COUNT);
                let samples = vec![0.0; n_samples as usize];
                let size = [n_samples, 1];
                let format = DEFAULT_AUDIO_TEXTURE_FORMAT;
                let texture = create_black_texture(device, encoder, size, format);
                IsfInputData::Audio { samples, texture }
            }
            isf::InputType::AudioFft(a) => {
                let n_columns = a.num_columns.unwrap_or(DEFAULT_AUDIO_FFT_COLUMNS);
                let columns = vec![0.0; n_columns as usize];
                let size = [n_columns, 1];
                let format = DEFAULT_AUDIO_TEXTURE_FORMAT;
                let texture = create_black_texture(device, encoder, size, format);
                IsfInputData::AudioFft { columns, texture }
            }
        }
    }

    /// Short-hand for checking that the input type matches the data.
    ///
    /// This is useful for checking to see if the user has changed the type of data associated with
    /// the name.
    pub fn ty_matches(&self, ty: &isf::InputType) -> bool {
        match (self, ty) {
            (IsfInputData::Event { .. }, isf::InputType::Event)
            | (IsfInputData::Bool(_), isf::InputType::Bool(_))
            | (IsfInputData::Long(_), isf::InputType::Long(_))
            | (IsfInputData::Float(_), isf::InputType::Float(_))
            | (IsfInputData::Point2d(_), isf::InputType::Point2d(_))
            | (IsfInputData::Color(_), isf::InputType::Color(_))
            | (IsfInputData::Image(_), isf::InputType::Image)
            | (IsfInputData::Audio { .. }, isf::InputType::Audio(_))
            | (IsfInputData::AudioFft { .. }, isf::InputType::AudioFft(_)) => true,
            _ => false,
        }
    }

    /// Update an existing instance ISF input data instance with the given input.
    fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        image_loader: &ImageLoader,
        images_path: &Path,
        input: &isf::Input,
    ) {
        match (self, &input.ty) {
            (IsfInputData::Event { .. }, isf::InputType::Event) => (),
            (IsfInputData::Bool(_), isf::InputType::Bool(_)) => (),
            (IsfInputData::Long(_), isf::InputType::Long(_)) => {}
            (IsfInputData::Float(_), isf::InputType::Float(_)) => {}
            (IsfInputData::Point2d(_), isf::InputType::Point2d(_)) => {}
            (IsfInputData::Color(_), isf::InputType::Color(_)) => {}
            (IsfInputData::Image(ref mut state), isf::InputType::Image) => {
                if let Some(img_path) = image_paths(images_path).next() {
                    state.update(device, encoder, image_loader, img_path);
                }
            }
            (IsfInputData::Audio { .. }, isf::InputType::Audio(_)) => {}
            (IsfInputData::AudioFft { .. }, isf::InputType::AudioFft(_)) => {}
            (data, _) => *data = Self::new(device, encoder, image_loader, images_path, input),
        }
    }
}

/// Created directly after successfully parsing an `Isf`.
///
/// `imported` textures can be accessed by the user.
#[derive(Debug, Default)]
pub struct IsfData {
    imported: HashMap<ImportName, ImageState>,
    inputs: HashMap<InputName, IsfInputData>,
    passes: Vec<wgpu::Texture>,
}

impl IsfData {
    /// The map of imported images.
    pub fn imported(&self) -> &HashMap<ImportName, ImageState> {
        &self.imported
    }

    /// The map of all declared inputs.
    pub fn inputs(&self) -> &HashMap<InputName, IsfInputData> {
        &self.inputs
    }

    /// The mutable map of all declared inputs.
    pub fn inputs_mut(&mut self) -> &mut HashMap<InputName, IsfInputData> {
        &mut self.inputs
    }

    /// The texture stored for each pass.
    pub fn passes(&self) -> &[wgpu::Texture] {
        &self.passes
    }
}

/// Ensure the image state map is up to date.
/// Update the GPU with new data.
pub fn sync_isf_data(
    device: &wgpu::Device,
    encoder: &mut wgpu::CommandEncoder,
    isf: &isf::Isf,
    output_attachment_size: [u32; 2],
    image_loader: &ImageLoader,
    images_path: &Path,
    isf_data: &mut IsfData,
) {
    // Update imported images.
    isf_data
        .imported
        .retain(|name, _| isf.imported.contains_key(name));
    for (key, img) in &isf.imported {
        let state = isf_data
            .imported
            .entry(key.clone())
            .or_insert(ImageState::None);
        state.update(device, encoder, image_loader, img.path.clone());
    }

    // Check all imported textures are loading.
    isf_data
        .inputs
        .retain(|key, _| isf.inputs.iter().map(|i| &i.name).any(|n| n == key));
    for input in &isf.inputs {
        let input_data = isf_data
            .inputs
            .entry(input.name.clone())
            .or_insert_with(|| {
                IsfInputData::new(device, encoder, image_loader, images_path, input)
            });
        input_data.update(device, encoder, image_loader, images_path, input);
    }

    // Prepare the textures that will be written to for passes.
    isf_data.passes.resize_with(isf.passes.len(), || {
        let texture = wgpu::TextureBuilder::new()
            .format(Frame::TEXTURE_FORMAT)
            .size(output_attachment_size)
            .usage(default_isf_texture_usage())
            .build(device);
        let data = vec![0u8; texture.size_bytes()];
        texture.upload_data(device, encoder, &data);
        texture
    });
}

// All textures stored within the `IsfData` instance in the order that they should be declared in
// the order expected by the isf textures bind group.
pub fn isf_data_textures(isf_data: &IsfData) -> impl Iterator<Item = &wgpu::Texture> {
    let imported = isf_data.imported.values().filter_map(|state| match state {
        ImageState::Ready(ref img_res) => match img_res {
            Ok(ref img_data) => Some(&img_data.texture),
            _ => None,
        },
        _ => None,
    });
    let inputs = isf_data
        .inputs
        .values()
        .filter_map(|input_data| match input_data {
            IsfInputData::Image(ref img_state) => match *img_state {
                ImageState::Ready(Ok(ref data)) => Some(&data.texture),
                _ => None,
            },
            IsfInputData::Audio { ref texture, .. }
            | IsfInputData::AudioFft { ref texture, .. } => Some(texture),
            _ => None,
        });
    let passes = isf_data.passes.iter();
    imported.chain(inputs).chain(passes)
}

/// The first set of ISF uniforms that are available to every ISF shader.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct IsfUniforms {
    pub date: [f32; 4],
    pub render_size: [f32; 2],
    pub pass_index: i32,
    pub time: f32,
    pub time_delta: f32,
    pub frame_index: i32,
}

pub type IsfInputUniforms = [u32; 128];
