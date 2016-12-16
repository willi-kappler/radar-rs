//! radar-rs: processing radar data with Rust
//!
//! Written by Willi Kappler, Version 0.1 (2016.12.15)
//!
//! Repository: https://github.com/willi-kappler/radar-rs
//!
//! License: MIT
//!

// For clippy
// #![feature(plugin)]
//
// #![plugin(clippy)]

// 'error_chain!' can recurse deeply
#![recursion_limit = "1024"]

#[macro_use] extern crate log;
extern crate jobsteal;
#[macro_use] extern crate error_chain;
extern crate chrono;
