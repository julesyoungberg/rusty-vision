# rusty vision

A live GLSL coding environment built with Rust, Nannou, and wgpu.

## features

- Interactive Shader Format (ISF)
- Multipass rendering
- Audio FFT
- Webcam
- Video & Image files

## setup

Install Rust and Cargo, clone the repo, then run

```
cargo run --release
```

For Windows, you need to install ASIO as described here: https://crates.io/crates/cpal

## controls

- Window size:
  - **1**: 852x480
  - **2**: 1280x720
  - **3**: 1920x1080
  - **4**: 2560x1440
  - **5**: 3840x2160
  - **0**: original
  - **H**: show / hide controls
  - **P**: pause / unpause
  - **R**: reset time to 0

## adding shaders

This rust application listens to the shaders directory (`shaders`), recompiling whenever changes are made. Shaders can be added to a subdirectory of `shaders` and referenced in the directory's `index.json` to be included in the UI's menu.

### program config

Descriptions of values you must configure in `index.json` for each program:

- `pipeline`: defines the shaders that make up the GPU pipeline. `frag` is required, and should be relative to the containing directory.

- `uniforms`: An array of uniform buffer lists for each program. Each list is a single string with uniform types separated with a comma.

- `config`: Default uniform values for each program.

- `isf`: If this is `true` the shader is expected to meet the ISF specification. In this case `uniforms` and `config` are ignored, and all configuration is provided in the shader. See https://github.com/mrRay/ISF_Spec.

## screenshots

An example of how the app handles errors in your shaders:

![](media/screenshot.png)

## architecture

This app manages data flow from the CPU to GPU as a collection of uniform buffers. A program can subscribe to any set of these uniform buffers by specifying so in the fragment shader and the config (`config.json`). A uniform could be any sort of data (e.g. 3D camera config, audio, webcam, images) that you might use as input to a GLSL sketch. New uniforms can be added by creating a new file in `src/programs/uniforms` similar in structure to `camera.rs` and `general.rs` for example. Then the new uniform can be made available by putting it to use in `src/programs/uniforms/mod.rs`, similarly to how `general` is used. Shaders can now subscribe to this data, but in order for the end user to interact with the system in real time, the uniforms must be controlled in some way.

The UI is heirarchy of functions starting with `interface::update()`. Different sections tend to correspond to different uniforms, and can be easily hidden or shown based on the current subscription. There are a few examples of this with the General and Geometry folders, as well as how the camera info box is hidden if camera data isn't needed.
