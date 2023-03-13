# Rust HxA mesh parser

## Description

This is a WIP parser for the mesh format specified by Eskil Steenberg.   
[Link to HxA GitHub](https://github.com/quelsolaar/HxA)

## Features

- Loading HxA files into Rust structures
- Searching through HxA Rust structures to obtain data

## Examples

To load a hxa file use the `from()` function and pass a filepath.    
```rust
use hxa;
let my_hxa = hxa::HXAFile::from("Cube.hxa");
```

You can parse this as is, or use the find functions to quickly obtain data:
```rust
use hxa::conventions::{hard,soft};

let model_geometry = my_hxa.get_first_geometry()
    .expect("Expected to find a geometry node").0;

let vertex_stack = &model_geometry.vertex_stack;

let vertex_positions = vertex_stack
    .find(hard::BASE_VERTEX_LAYER_NAME)
    .expect("Expected to find a vertex layer")
    .as_vec_f32();

let vertex_normals = vertex_stack
    .find(soft::LAYER_NORMALS)
    .expect("Expected to find a normal layer")
    .as_vec_f32();
```

## Missing features
- Several parsing types are not implemented, however the essental ones are.