// a fork of https://github.com/nannou-org/nannou/blob/master/nannou_isf/src/pipeline.rs

use nannou::image;
use nannou::prelude::*;
use opencv::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use thiserror::Error;
use threadpool::ThreadPool;
use tinyfiledialogs::open_file_dialog;

use crate::programs::uniforms::audio::AudioUniforms;
use crate::programs::uniforms::audio_fft::AudioFftUniforms;
use crate::programs::uniforms::audio_source::AudioSource;
use crate::programs::uniforms::video_capture::VideoCapture;

pub const DEFAULT_AUDIO_SAMPLE_COUNT: u32 = 64;
pub const DEFAULT_AUDIO_FFT_COLUMNS: u32 = 64;

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

#[derive(Debug)]
pub struct LoadingImage {
    receiver: mpsc::Receiver<Result<image::RgbaImage, ImageLoadError>>,
    texture: wgpu::Texture,
}

/// The state of the image.
#[derive(Debug)]
pub enum ImageState {
    None,
    Loading(LoadingImage),
    Ready(Result<ImageData, ImageLoadError>),
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

impl ImageState {
    /// Update the image state.
    fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        image_loader: &ImageLoader,
        img_path: PathBuf,
    ) -> bool {
        match *self {
            ImageState::None => {
                let (tx, rx) = mpsc::channel();

                image_loader.threadpool.execute(move || {
                    println!("loading {:?}", img_path);
                    let img_res = image::open(img_path)
                        .map(|img| img.to_rgba8())
                        .map_err(|err| err.into());
                    tx.send(img_res).ok();
                });

                let texture = create_black_texture(
                    device,
                    encoder,
                    [1_u32, 1_u32],
                    wgpu::TextureFormat::R8Unorm,
                );

                *self = ImageState::Loading(LoadingImage {
                    receiver: rx,
                    texture,
                });

                true
            }
            ImageState::Loading(ref loading_image) => match loading_image.receiver.try_recv() {
                Ok(img_res) => {
                    let usage = wgpu::TextureUsage::COPY_DST | wgpu::TextureUsage::SAMPLED;

                    let res = img_res.map(|image| {
                        let texture = wgpu::Texture::encode_load_from_image_buffer(
                            device, encoder, usage, &image,
                        );
                        ImageData { image, texture }
                    });

                    println!("loaded: {:?}", img_path);
                    *self = ImageState::Ready(res);

                    true
                }
                _ => false,
            },
            ImageState::Ready(_) => false,
        }
    }

    pub fn get_error(&self) -> Option<&ImageLoadError> {
        match self {
            ImageState::Ready(result) => match result {
                Err(error) => Some(error),
                _ => None,
            },
            _ => None,
        }
    }
}

pub struct IsfInputError {
    msg: String,
    ty: String,
}

#[derive(Debug)]
pub enum ImageSource {
    None,
    Image(ImageState),
    Video(VideoCapture),
    Webcam(VideoCapture),
}

#[derive(Debug)]
pub struct ImageInput {
    pub source: ImageSource,
}

impl ImageInput {
    fn new() -> Self {
        Self {
            source: ImageSource::None,
        }
    }

    fn end_sessions(&mut self) {
        match &mut self.source {
            ImageSource::Video(v) => v.end_session(),
            ImageSource::Webcam(v) => v.end_session(),
            _ => (),
        };
    }

    pub fn load_image(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        image_loader: &ImageLoader,
        path: PathBuf,
    ) -> bool {
        self.end_sessions();
        let mut image_source = ImageState::None;
        let updated = image_source.update(device, encoder, image_loader, path);
        self.source = ImageSource::Image(image_source);
        updated
    }

    pub fn select_image(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        image_loader: &ImageLoader,
    ) {
        let filepath = match open_file_dialog("Load Image", "~", Some((&["*.jpg", "*.png"], ""))) {
            Some(path) => path,
            None => return,
        };

        println!("selected image: {:?}", filepath);

        self.load_image(device, encoder, image_loader, PathBuf::from(filepath));
    }

    pub fn select_video(&mut self, device: &wgpu::Device) {
        let filepath = match open_file_dialog(
            "Load Video",
            "~",
            Some((&["*.mp4", "*.avi", "*.mov", "*.mpeg", "*.flv", "*.wmv"], "")),
        ) {
            Some(filepath) => filepath,
            None => return,
        };

        println!("selected video: {:?}", filepath);

        self.end_sessions();

        let capture =
            opencv::videoio::VideoCapture::from_file(&filepath, opencv::videoio::CAP_ANY).unwrap();

        let video_capture = VideoCapture::new(device, capture, 1.0);

        self.source = ImageSource::Video(video_capture);
    }

    pub fn start_webcam(&mut self, device: &wgpu::Device, size: Point2) {
        println!("selected webcam");

        self.end_sessions();

        let mut capture = opencv::videoio::VideoCapture::new(0, opencv::videoio::CAP_ANY).unwrap();
        capture
            .set(opencv::videoio::CAP_PROP_FRAME_WIDTH, size[0] as f64)
            .ok();
        capture
            .set(opencv::videoio::CAP_PROP_FRAME_HEIGHT, size[1] as f64)
            .ok();

        let video_capture = VideoCapture::new(device, capture, 1.0);

        self.source = ImageSource::Webcam(video_capture);
    }

    pub fn get_error(&self) -> Option<IsfInputError> {
        match &self.source {
            ImageSource::Image(image_state) => match image_state.get_error() {
                Some(error) => Some(IsfInputError {
                    msg: error.to_string(),
                    ty: String::from("Image"),
                }),
                _ => None,
            },
            ImageSource::Video(capture) => match &capture.error {
                Some(error) => Some(IsfInputError {
                    msg: error.clone(),
                    ty: String::from("Video"),
                }),
                None => None,
            },
            ImageSource::Webcam(capture) => match &capture.error {
                Some(error) => Some(IsfInputError {
                    msg: error.clone(),
                    ty: String::from("Webcam"),
                }),
                None => None,
            },
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum IsfInputData {
    Event { happening: bool },
    Bool(bool),
    Long { value: i32, selected: usize },
    Float(f32),
    Point2d(Point2),
    Color(LinSrgba),
    Image(ImageInput),
    Audio(AudioUniforms),
    AudioFft(AudioFftUniforms),
}

/// Given a path to a directory, produces the paths of all images within it.
fn image_paths(dir: &Path) -> impl Iterator<Item = PathBuf> {
    walkdir::WalkDir::new(dir)
        .into_iter()
        .filter_map(|res| res.ok())
        .map(|entry| entry.path().to_path_buf())
        .filter(|path| image::image_dimensions(path).ok().is_some())
}

impl IsfInputData {
    /// Initialise a new `IsfInputData` instance.
    fn new(
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        image_loader: &ImageLoader,
        images_path: &Path,
        audio_source: &mut AudioSource,
        input: &isf::Input,
        _size: [u32; 2],
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
                let index = n.values.iter().position(|v| *v == init).unwrap_or(0);
                IsfInputData::Long {
                    value: init,
                    selected: index,
                }
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
                let red = v.get(0).cloned().unwrap_or_default();
                let green = v.get(1).cloned().unwrap_or_default();
                let blue = v.get(2).cloned().unwrap_or_default();
                let alpha = v.get(3).cloned().unwrap_or_default();
                IsfInputData::Color(lin_srgba(red, green, blue, alpha))
            }
            isf::InputType::Image => {
                let mut image_input = ImageInput::new();
                // mage_input.start_webcam(device, pt2(size[0] as f32, size[1] as f32));
                if let Some(path) = image_paths(images_path).next() {
                    image_input.load_image(device, encoder, image_loader, path);
                }
                IsfInputData::Image(image_input)
            }
            isf::InputType::Audio(a) => {
                let n_samples = a.num_samples.unwrap_or(DEFAULT_AUDIO_SAMPLE_COUNT);
                let mut audio = AudioUniforms::new(device, Some(n_samples as usize));
                audio.start_session(audio_source);
                IsfInputData::Audio(audio)
            }
            isf::InputType::AudioFft(a) => {
                let n_columns = a.num_columns.unwrap_or(DEFAULT_AUDIO_FFT_COLUMNS);
                let mut audio_fft = AudioFftUniforms::new(device, Some(n_columns as usize));
                audio_fft.smoothing = 0.0;
                audio_fft.start_session(audio_source);
                IsfInputData::AudioFft(audio_fft)
            }
        }
    }

    /// Update an existing instance ISF input data instance with the given input.
    fn update(
        &mut self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        image_loader: &ImageLoader,
        images_path: &Path,
        audio_source: &mut AudioSource,
        input: &isf::Input,
        size: [u32; 2],
    ) -> bool {
        match (self, &input.ty) {
            (IsfInputData::Event { .. }, isf::InputType::Event) => (),
            (IsfInputData::Bool(_), isf::InputType::Bool(_)) => (),
            (IsfInputData::Long { .. }, isf::InputType::Long(_)) => {}
            (IsfInputData::Float(_), isf::InputType::Float(_)) => {}
            (IsfInputData::Point2d(_), isf::InputType::Point2d(_)) => {}
            (IsfInputData::Color(_), isf::InputType::Color(_)) => {}
            (IsfInputData::Image(ref mut image_input), isf::InputType::Image) => {
                match &mut image_input.source {
                    ImageSource::None => {
                        if let Some(path) = image_paths(images_path).next() {
                            return image_input.load_image(device, encoder, image_loader, path);
                        }
                    }
                    ImageSource::Image(image_state) => {
                        if let Some(path) = image_paths(images_path).next() {
                            return image_state.update(device, encoder, image_loader, path);
                        }
                    }
                    ImageSource::Video(ref mut video) | ImageSource::Webcam(ref mut video) => {
                        video.update();
                        video.update_texture(device, encoder);
                    }
                }
            }
            (IsfInputData::Audio(audio), isf::InputType::Audio(_)) => {
                audio.update();
                audio.update_texture(device, encoder);
            }
            (IsfInputData::AudioFft(audio_fft), isf::InputType::AudioFft(_)) => {
                audio_fft.update();
                audio_fft.update_texture(device, encoder);
            }
            (data, _) => {
                *data = Self::new(
                    device,
                    encoder,
                    image_loader,
                    images_path,
                    audio_source,
                    input,
                    size,
                )
            }
        }
        false
    }

    fn end_session(&mut self, audio_source: &mut AudioSource) {
        match self {
            IsfInputData::Image(ref mut image_input) => match &mut image_input.source {
                ImageSource::Video(ref mut video) | ImageSource::Webcam(ref mut video) => {
                    video.end_session();
                }
                _ => (),
            },
            IsfInputData::Audio(audio) => {
                audio.end_session(audio_source);
            }
            IsfInputData::AudioFft(audio_fft) => {
                audio_fft.end_session(audio_source);
            }
            _ => (),
        }
    }

    fn pause(&mut self, audio_source: &mut AudioSource) {
        match self {
            IsfInputData::Image(ref mut image_input) => match &mut image_input.source {
                ImageSource::Video(ref mut video) | ImageSource::Webcam(ref mut video) => {
                    video.pause();
                }
                _ => (),
            },
            IsfInputData::Audio(audio) => {
                audio.end_session(audio_source);
            }
            IsfInputData::AudioFft(audio_fft) => {
                audio_fft.end_session(audio_source);
            }
            _ => (),
        }
    }

    fn unpause(&mut self, audio_source: &mut AudioSource) {
        match self {
            IsfInputData::Image(ref mut image_input) => match &mut image_input.source {
                ImageSource::Video(ref mut video) | ImageSource::Webcam(ref mut video) => {
                    video.unpause();
                }
                _ => (),
            },
            IsfInputData::Audio(audio) => {
                audio.start_session(audio_source);
            }
            IsfInputData::AudioFft(audio_fft) => {
                audio_fft.start_session(audio_source);
            }
            _ => (),
        }
    }

    fn get_error(&self) -> Option<IsfInputError> {
        match self {
            IsfInputData::Image(ref image_input) => image_input.get_error(),
            _ => None,
        }
    }
}

pub type IsfDataInputs = HashMap<InputName, IsfInputData>;

#[derive(Debug, Clone)]
pub struct IsfPassTextures {
    pub uniform_texture: wgpu::Texture,
    pub render_texture: wgpu::Texture,
}

impl IsfPassTextures {
    pub fn new(
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        size: [u32; 2],
        num_samples: u32,
    ) -> Self {
        let render_texture = wgpu::TextureBuilder::new()
            .format(Frame::TEXTURE_FORMAT)
            .size(size)
            .usage(
                wgpu::TextureUsage::OUTPUT_ATTACHMENT
                    | wgpu::TextureUsage::COPY_SRC
                    | wgpu::TextureUsage::COPY_DST,
            )
            .sample_count(num_samples)
            .build(device);

        let uniform_texture = wgpu::TextureBuilder::new()
            .format(Frame::TEXTURE_FORMAT)
            .size(size)
            .usage(default_isf_texture_usage())
            .sample_count(num_samples)
            .build(device);

        let data = vec![0u8; uniform_texture.size_bytes()];
        uniform_texture.upload_data(device, encoder, &data);
        render_texture.upload_data(device, encoder, &data);

        Self {
            render_texture,
            uniform_texture,
        }
    }

    pub fn size(&self) -> [u32; 2] {
        self.uniform_texture.size()
    }

    pub fn size_bytes(&self) -> usize {
        self.uniform_texture.size_bytes()
    }

    pub fn upload_data(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        data: &[u8],
    ) {
        self.uniform_texture.upload_data(device, encoder, data);
        self.render_texture.upload_data(device, encoder, data);
    }
}

/// Created directly after successfully parsing an `Isf`.
///
/// `imported` textures can be accessed by the user.
#[derive(Debug, Default)]
pub struct IsfData {
    imported: HashMap<ImportName, ImageState>,
    inputs: IsfDataInputs,
    passes: Vec<IsfPassTextures>,
}

impl IsfData {
    /// The map of imported images.
    pub fn imported(&self) -> &HashMap<ImportName, ImageState> {
        &self.imported
    }

    /// The map of all declared inputs.
    pub fn inputs(&self) -> &IsfDataInputs {
        &self.inputs
    }

    /// The mutable map of all declared inputs.
    pub fn inputs_mut(&mut self) -> &mut IsfDataInputs {
        &mut self.inputs
    }

    /// The texture stored for each pass.
    pub fn passes(&self) -> &[IsfPassTextures] {
        &self.passes
    }

    pub fn end_session(&mut self, audio_source: &mut AudioSource) {
        self.inputs.iter_mut().for_each(|(_, input)| {
            input.end_session(audio_source);
        });
    }

    pub fn pause(&mut self, audio_source: &mut AudioSource) {
        self.inputs
            .iter_mut()
            .for_each(|(_, input)| input.pause(audio_source));
    }

    pub fn unpause(&mut self, audio_source: &mut AudioSource) {
        self.inputs
            .iter_mut()
            .for_each(|(_, input)| input.unpause(audio_source));
    }

    pub fn get_render_texture(&self, index: usize) -> &wgpu::Texture {
        &self.passes[index].render_texture
    }

    pub fn get_final_texture(&self) -> Option<&wgpu::Texture> {
        match self.passes.last() {
            Some(last) => Some(&last.uniform_texture),
            None => None,
        }
    }

    pub fn get_errors(&self) -> HashMap<String, Vec<String>> {
        let mut errors = HashMap::new();
        let image_key = String::from("Image");

        self.imported.iter().for_each(|(_, image)| {
            if let Some(error) = image.get_error() {
                let entry = errors.entry(image_key.clone()).or_insert_with(Vec::new);
                entry.push(error.to_string());
            }
        });

        self.inputs.iter().for_each(|(_, input)| {
            if let Some(error) = input.get_error() {
                let entry = errors.entry(error.ty).or_insert_with(Vec::new);
                entry.push(error.msg);
            }
        });

        errors
    }
}

pub fn evaluate_dimension_equation(
    equation: &str,
    base_size: [u32; 2],
    isf_data: &mut IsfData,
) -> Option<u32> {
    let re = Regex::new(r"\$(\w+)").unwrap();
    let subbed_equation = re
        .replace_all(equation, |captures: &regex::Captures| {
            let var_name = &captures[1];

            match var_name {
                "WIDTH" => return format!("{}", base_size[0]),
                "HEIGHT" => return format!("{}", base_size[1]),
                _ => (),
            };

            if let Some(input) = isf_data.inputs.get(var_name) {
                match input {
                    IsfInputData::Float(val) => {
                        return format!("{}", val);
                    }
                    IsfInputData::Long { value, .. } => {
                        return format!("{}", value);
                    }
                    _ => (),
                };
            }

            String::from("0")
        })
        .to_string();

    match mexprp::eval::<f64>(subbed_equation.as_str()) {
        Ok(result) => match result {
            mexprp::Answer::Single(val) => Some(val.round() as u32),
            _ => None,
        },
        _ => None,
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
    audio_source: &mut AudioSource,
    isf_data: &mut IsfData,
    num_samples: u32,
) -> bool {
    let mut textures_updated = false;

    // Update imported images.
    isf_data
        .imported
        .retain(|name, _| isf.imported.contains_key(name));
    for (key, img) in &isf.imported {
        let path = images_path.join(img.path.clone());
        let state = isf_data
            .imported
            .entry(key.clone())
            .or_insert(ImageState::None);

        if state.update(device, encoder, image_loader, path) {
            textures_updated = true;
        }
    }

    // Remove old inputs - do any cleanup here
    isf_data.inputs.retain(|key, input| {
        let keep = isf.inputs.iter().map(|i| &i.name).any(|n| n == key);
        if !keep {
            input.end_session(audio_source);
        }
        keep
    });

    // Update input data
    for input in &isf.inputs {
        let input_data = isf_data
            .inputs
            .entry(input.name.clone())
            .or_insert_with(|| {
                IsfInputData::new(
                    device,
                    encoder,
                    image_loader,
                    images_path,
                    audio_source,
                    input,
                    output_attachment_size,
                )
            });
        if input_data.update(
            device,
            encoder,
            image_loader,
            images_path,
            audio_source,
            input,
            output_attachment_size,
        ) {
            textures_updated = true;
        }
    }

    // Prepare the textures that will be written to for passes.
    let mut passes = isf_data.passes().to_owned();
    isf_data.passes = vec![];

    for p in isf.passes.iter() {
        let mut width = output_attachment_size[0];
        let mut height = output_attachment_size[1];

        if let Some(width_eq) = &p.width {
            if let Some(w) = evaluate_dimension_equation(width_eq, output_attachment_size, isf_data)
            {
                width = w;
            }
        }

        if let Some(height_eq) = &p.height {
            if let Some(h) =
                evaluate_dimension_equation(height_eq, output_attachment_size, isf_data)
            {
                height = h;
            }
        }

        // if a texture already exists and the size hasn't changed, return that
        if !passes.is_empty() {
            let pass_textures = passes.remove(0);
            let size = pass_textures.size();
            if size[0] == width && size[1] == height {
                if !p.persistent {
                    // clear the texture if it isn't persistent
                    let data = vec![0u8; pass_textures.size_bytes()];
                    pass_textures.upload_data(device, encoder, &data);
                }

                isf_data.passes.push(pass_textures);
                continue;
            }
        }

        isf_data.passes.push(IsfPassTextures::new(
            device,
            encoder,
            [width, height],
            num_samples,
        ));
    }

    textures_updated
}

// All textures stored within the `IsfData` instance in the order that they should be declared in
// the order expected by the isf textures bind group.
pub fn isf_data_textures<'a>(isf_data: &'a IsfData, isf: &'a isf::Isf) -> Vec<&'a wgpu::Texture> {
    let mut textures = vec![];

    let imported_data = isf_data.imported();
    for (key, _) in &isf.imported {
        let state = match imported_data.get(key) {
            Some(img) => img,
            None => continue,
        };

        let texture = match state {
            ImageState::Ready(ref img_res) => match img_res {
                Ok(ref img_data) => &img_data.texture,
                Err(_) => continue,
            },
            ImageState::Loading(ref loading_image) => &loading_image.texture,
            _ => continue,
        };

        textures.push(texture);
    }

    let input_data = isf_data.inputs();
    for i in &isf.inputs {
        let input = match input_data.get(&i.name) {
            Some(input) => input,
            None => continue,
        };

        let texture = match input {
            IsfInputData::Image(ref img_input) => match &img_input.source {
                ImageSource::Image(ref image_state) => match &image_state {
                    ImageState::Ready(Ok(ref data)) => &data.texture,
                    ImageState::Loading(ref loading_image) => &loading_image.texture,
                    _ => continue,
                },
                ImageSource::Video(ref video) | ImageSource::Webcam(ref video) => {
                    &video.video_texture
                }
                _ => continue,
            },
            IsfInputData::Audio(audio) => &audio.audio_texture,
            IsfInputData::AudioFft(audio_fft) => &audio_fft.spectrum_texture,
            _ => continue,
        };

        textures.push(texture);
    }

    for pass in &isf_data.passes {
        textures.push(&&pass.uniform_texture);
    }

    textures
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

fn float_as_bytes(data: &f32) -> &[u8] {
    unsafe { wgpu::bytes::from(data) }
}

fn int_as_bytes(data: &i32) -> &[u8] {
    unsafe { wgpu::bytes::from(data) }
}

fn point_as_bytes(data: &Vector2<f32>) -> &[u8] {
    unsafe { wgpu::bytes::from(data) }
}

fn color_as_bytes(data: &LinSrgba) -> &[u8] {
    unsafe { wgpu::bytes::from(data) }
}

pub fn get_isf_input_uniforms_bytes_vec(isf_opt: &Option<isf::Isf>, isf_data: &IsfData) -> Vec<u8> {
    let isf = match isf_opt {
        Some(i) => i,
        None => return vec![],
    };

    let data_inputs = isf_data.inputs();
    let mut bytes = vec![];

    for input in &isf.inputs {
        let data = data_inputs.get(&input.name).unwrap();
        match data {
            IsfInputData::Float(val) => bytes.extend(float_as_bytes(val)),
            IsfInputData::Long { value, .. } => bytes.extend(int_as_bytes(value)),
            IsfInputData::Point2d(point) => bytes.extend(point_as_bytes(point)),
            IsfInputData::Color(color) => bytes.extend(color_as_bytes(color)),
            _ => (),
        }
    }

    bytes
}
