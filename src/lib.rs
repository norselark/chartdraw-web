#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::non_ascii_literal
)]
#![recursion_limit = "512"]

#[macro_use]
extern crate lazy_static;

mod app;
mod components;
mod input;
mod optimize;

use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() {
    yew::start_app::<app::App>();
}
