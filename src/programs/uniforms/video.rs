use nannou::prelude::*;

use crate::app;
use crate::programs::config;
use crate::programs::uniforms::base::Bufferable;
use crate::programs::uniforms::video_capture::VideoCapture;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Data {
    pub video_size: Vector2,
}

pub struct VideoUniforms {
    pub updated: bool,
    pub video_capture: Option<VideoCapture>,
    pub video_path: Option<String>,

    data: Data,
}

impl Bufferable<Data> for VideoUniforms {
    fn as_bytes(&self) -> &[u8] {
        unsafe { wgpu::bytes::from(&self.data) }
    }

    fn textures(&self) -> Vec<&wgpu::Texture> {
        match &self.video_capture {
            Some(capture) => vec![&capture.video_texture],
            None => vec![],
        }
    }
}

impl VideoUniforms {
    pub fn new() -> Self {
        Self {
            data: Data {
                video_size: pt2(0.0, 0.0),
            },
            updated: false,
            video_capture: None,
            video_path: None,
        }
    }

    pub fn configure(
        &mut self,
        app: &App,
        device: &wgpu::Device,
        settings: &Option<config::ProgramSettings>,
    ) {
        if let Some(cnfg) = settings {
            let project_path = app.project_path().expect("failed to locate `project_path`");

            if let Some(video) = &cnfg.video {
                self.video_path = Some(
                    project_path
                        .join(app::MEDIA_DIR)
                        .join(video)
                        .into_os_string()
                        .into_string()
                        .unwrap(),
                );

                self.start_session(device);
            }
        }
    }

    fn start_session(&mut self, device: &wgpu::Device) {
        let video_path = match self.video_path.clone() {
            Some(p) => p,
            None => return,
        };

        let capture =
            opencv::videoio::VideoCapture::from_file(video_path.as_str(), opencv::videoio::CAP_ANY)
                .unwrap();

        self.video_capture = Some(VideoCapture::new(device, capture));

        self.updated = true;
    }

    pub fn end_session(&mut self) {
        if let Some(video_capture) = &mut self.video_capture {
            video_capture.end_session();
            self.video_capture = None;
        }
    }

    pub fn update(&mut self) {
        if let Some(video_capture) = &mut self.video_capture {
            video_capture.update();
            self.data.video_size = video_capture.video_size;
        }
    }

    pub fn update_texture(&self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder) {
        if let Some(video_capture) = &self.video_capture {
            video_capture.update_texture(device, encoder);
        }
    }

    pub fn pause(&mut self) {
        if let Some(video_capture) = &mut self.video_capture {
            video_capture.pause();
        }
    }

    pub fn unpause(&mut self) {
        if let Some(video_capture) = &mut self.video_capture {
            video_capture.unpause();
        }
    }
}
