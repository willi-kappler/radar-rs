//! radar-rs: processing radar data with Rust
//!
//! Written by Willi Kappler, Version 0.1 (2016.12.15)
//!
//! Repository: https://github.com/willi-kappler/radar-rs
//!
//! License: MIT
//!

use chrono::date::Date;

enum ImageFormat {
    FLOAT,
    FCOMPLEX,
}

enum ImageGeometry {
    SLANT_RANGE,
}

struct GammaMetadata {
    title: String,
    sensor: String,
    date: Date,
    start_time: f64,
    center_time: f64,
    end_time: f64,
    azimuth_line_time: f64,
    line_header_size: u32,
    width: usize,
    height: usize,
    range_looks: u32,
    azimuth_looks: u32,
    image_geometry: ImageGeometry,
    range_scale: f64,
    azimuth_scale: f64,
    // TODO: add more metadata items
}

struct FloatPixel {
    value: f64
}

struct ComplexPixel {
    real: f64,
    image: f64
}

trait GammaPixel {

}

struct GammaRadarData<T: GammaPixel> {
    data: Vec<T>,
    metadata: GammaMetadata
}
