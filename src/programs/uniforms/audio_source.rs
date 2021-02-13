use cpal;

pub enum AudioMessage {
    Data(Vec<f32>),
    Close(()),
}

pub struct AudioSource {
    channels: Vec<Sender<AudioMessage>>,
    stream: Option<cpal::Stream>,
}

impl AudioSource {
    pub fn new() -> Self {
        Self {}
    }
}
