// use cpal;

// pub enum AudioMessage {
//     Data(Vec<f32>),
//     Close(()),
// }

// pub struct AudioSource {
//     stream: Option<cpal::Stream>,
// }

// impl AudioSource {
//     pub fn new() -> Self {
//         Self { stream: None }
//     }

//     pub fn start_session(&mut self, subscribers: Vec<Sender<AudioMessage>>) {
//         // get default audio input device
//         let audio_device = match cpal::default_host().default_input_device() {
//             Some(device) => device,
//             None => {
//                 self.error = Some(String::from("Unable to connect to default audio device"));
//                 return false;
//             }
//         };

//         // find supported config
//         let supported_configs = match audio_device.supported_input_configs() {
//             Ok(mut configs) => match configs.next() {
//                 Some(config) => config,
//                 None => {
//                     self.error = Some(String::from("No audio configuration available"));
//                     return false;
//                 }
//             },
//             Err(e) => {
//                 self.error = Some(format!("Error configuring audio input: {:?}", e));
//                 return false;
//             }
//         };

//         let audio_config = supported_configs.with_max_sample_rate();
//         let cpal::SampleRate(sample_rate) = audio_config.sample_rate();
//     }
// }
