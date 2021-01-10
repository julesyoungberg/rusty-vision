# rusty vision

A GLSL creative coding environment built with Rust, Nannou, and WGPU.

## setup

Install Rust and Cargo, then run

```
cargo run --release
```

## controls

The camera can be moved with the arrow keys and rotated with WASD.

## adding shaders

Shaders can be added to `src/shaders` and referenced in `src/config.rs` to be included in the UI's menu. More specifically, to add a new shader program, you must modify the following constant configuration variables:

- `PIPELINES`: Shader pipeline descriptions (`[name, vertex_shader, frag_shader]`) with paths relative to `./src/shaders/`

- `PROGRAMS`: Program names, corresponding to `PIPELINES`

- `PROGRAM_DEFAULTS`: Default uniform values for each program.

- `PROGRAM_UNIFORMS`: An array of uniform buffer lists for each program. Each list is a single string with uniform types separated with a comma.

## screenshots

![](images/screenshot.png)

## architecture

This app manages data flow from the CPU to GPU as a collection of uniform buffers. A program can subscribe to any set of these uniform buffers by specifying so in the fragment shader and the config. A uniform could be any sort of data (e.g. 3D camera config, audio, webcam, images) that you might use as input to a GLSL sketch. New uniforms can be added by creating a new file in `src/programs/uniforms` similar in structure to `camera.rs`, `general.rs`, and `geometry.rs`. Then the new uniform can be made available by adding it to `src/programs/mod.rs`.

## development

Currently, most of the work to do is around developing different types of uniform buffers for shaders to 'subscribe' to (in `PROGRAM_UNIFORMS`). A system for doing this in place, but it now must be put to use.

Secondly, shader programs must be written to use this data.

### todos

- figure out why surfaces in the fractal raymarcher are made up of spheres and look bubbly. This is a regression and was not present in very early implementations of the shaders.
