# Nanachi - pure Rust 2D graphics library

[![Build Status](https://github.com/carrotflakes/nanachi/workflows/Rust/badge.svg)](https://github.com/carrotflakes/nanachi/actions)
[![Crates.io](https://img.shields.io/crates/v/nanachi.svg)](https://crates.io/crates/nanachi)
[![Documentation](https://docs.rs/nanachi/badge.svg)](https://docs.rs/nanachi)

**NOTE: Nanachi is still buggy and the API will changed!**

![nanachi](https://github.com/carrotflakes/nanachi/raw/master/nanachi.png)

Generated by `cargo run --release --example nanachi`

## Features
- path filling and stroking
- color with: linear gradients, radial gradients and patterns
- 24 composition types
- anti-aliasing (can be disabled)
- path transformation: translation, scaling and rotation
- image crate friendly interface (but not required)

## Example
Basic usage example is following:

``` rust
use image::RgbaImage;
use nanachi::{
    compositor,
    context::{Context, FillStyle},
    fill_color, fill_rule,
    path_builder::PathBuilder,
    pixel::Rgba,
};

let (width, height) = (512, 512);

// Make a Context
let mut context = Context::from_pixel(width, height, Rgba([1.0, 1.0, 1.0, 1.0])).high_quality();

// Make a Path
let mut builder = PathBuilder::new();
builder.move_to(100.0, 100.0);
builder.line_to(200.0, 100.0);
builder.line_to(200.0, 200.0);
builder.line_to(100.0, 200.0);
builder.close();
let path = builder.end();

// Make a FillStyle for filling
let fill_style = FillStyle::new(
    fill_color::Solid::new(Rgba([1.0, 0.0, 0.0, 0.7])),
    compositor::SrcOver,
    fill_rule::NonZero,
);

// Fill the path
context.fill(&path, &fill_style);

// Make a FillStyle for stroking
let fill_style = FillStyle::new(
    fill_color::Solid::new(Rgba([0.0, 0.0, 1.0, 1.0])),
    compositor::SrcOver,
    fill_rule::NonZero,
);

// Stroke the path
context.stroke(&path, &fill_style, 8.0);

// Save the image
let img: RgbaImage = (&context.image).into();
img.save("./basic.png").unwrap();
```

## Author

* carrotflakes (carrotflakes@gmail.com)

## Copyright

Copyright (c) 2020 carrotflakes (carrotflakes@gmail.com)

## License

Licensed under the MIT License.
