# rusty vision

A rust nannou application for generating and controlling graphics (primarily focused on fractals and ray marching).

Designed to be used as a creative coding environment and/or performance tool.

## setup

Install Rust and Cargo, then run

```
cargo run --release
```

## controls

The camera can be moved with the arrow keys and rotated with WASD.

## adding shaders

Shaders can be added to `src/shaders` and referenced in `src/config.rs` to be included in the UI's menu.

## screenshots

![](images/screenshot.png)
