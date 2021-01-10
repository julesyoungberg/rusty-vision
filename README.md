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

- `PIPELINES`: Shader pipeline descriptions (`[internal_name, vertex_shader, frag_shader]`)

- `PROGRAMS`: Program names, displayed to the user.

- `PROGRAM_DEFAULTS`: Default uniform values for each program.

- `PROGRAM_UNIFORMS`: An array of uniform buffer lists for each program. Each list is a single string with uniform types separated with a comma.

## screenshots

![](images/screenshot.png)
