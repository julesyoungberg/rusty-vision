# rusty vision

A live GLSL coding environment built with Rust, Nannou, and wgpu.

## setup

Install Rust and Cargo, clone the repo, then run

```
cargo run --release
```

For Windows, you need to install ASIO as described here: https://crates.io/crates/cpal

For audio visualization, get the mirlin server from the [github repo](https://github.com/julesyoungberg/mirlin-server) or [dockerhub](https://hub.docker.com/repository/docker/julesyoungberg/mirlin-server)

## controls

Controls vary depending on the current program.

For 3D programs, the camera can be moved with the arrow keys and rotated with WASD.

## adding shaders

This rust application listens to the shaders directory (`shaders`), recompiling whenever changes are made. Shaders can be added to `shaders` and referenced in `src/config.json` to be included in the UI's menu.

### program config

Descriptions of values you must configure in `config.json` for each program:

- `pipeline`: defines the shaders that make up the GPU pipeline. `vert` and `frag` are required, these paths should be relative to `./shaders/`.

- `uniforms`: An array of uniform buffer lists for each program. Each list is a single string with uniform types separated with a comma.

- `defaults`: Default uniform values for each program.

## screenshots

An example of how the app handles errors in your shaders:

![](images/screenshot.png)

## architecture

This app manages data flow from the CPU to GPU as a collection of uniform buffers. A program can subscribe to any set of these uniform buffers by specifying so in the fragment shader and the config (`config.json`). A uniform could be any sort of data (e.g. 3D camera config, audio, webcam, images) that you might use as input to a GLSL sketch. New uniforms can be added by creating a new file in `src/programs/uniforms` similar in structure to `camera.rs` and `general.rs` for example. Then the new uniform can be made available by putting it to use in `src/programs/uniforms/mod.rs`, similarly to how `general` is used. Shaders can now subscribe to this data, but in order for the end user to interact with the system in real time, the uniforms must be controlled in some way.

The UI is heirarchy of functions starting with `interface::update()`. Different sections tend to correspond to different uniforms, and can be easily hidden or shown based on the current subscription. There are a few examples of this with the General and Geometry folders, as well as how the camera info box is hidden if camera data isn't needed.

## roadmap

Currently, most of the work to do is around developing different types of uniform buffers for shaders to 'subscribe' to (in `config.json::programs[program].uniforms`), as well as breaking down the general uniforms into more specific groups. A system for doing this in place, but it now must be put to use.

Along with this work comes developing appropriate UI components to allow the creative coder to control the uniforms. Nannou has a very powerful UI system, and what is in `src/interface` only scratches the surface.

Lastly, shader programs must be written to use this data and create interesting experiences!
