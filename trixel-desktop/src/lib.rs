//! Trixel Rust Desktop App, built on top of the Trixel Core Library,
//! a FOSS trigonometric painting app, allowing you to paint
//! with triangles, pixels and hexagons.

/*
Trixel, a FOSS trigonometric painting app.
Copyright (C) 2021  Glen Henri J. De Cauwsemaeker and Elizabeth C. Gonzales Belsuzarri.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
*/

#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod painting;
pub use app::TrixelDesktopApp;

// ----------------------------------------------------------------------------
// When compiling for web:

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::{self, prelude::*};

/// This is the entry-point for all the web-assembly.
/// This is called once from the HTML.
/// It loads the app, installs some callbacks, then returns.
/// You can add more callbacks like this if you want to call in to your code.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn start(canvas_id: &str) -> Result<(), eframe::wasm_bindgen::JsValue> {
    let app = TrixelDesktopApp::default();
    eframe::start_web(canvas_id, Box::new(app))
}
