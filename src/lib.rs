#![warn(clippy::pedantic)]
#![recursion_limit="512"]

#[macro_use]
extern crate lazy_static;

mod app;
mod components;
mod input;
mod optimize;

use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn run_app() {
    yew::start_app::<app::App>();
}
