mod utils;
mod sliding_square;
mod tetris;
mod tetris_engine;

use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use web_sys::{CanvasRenderingContext2d, OffscreenCanvas, console};
use web_sys::{console};
// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref RENDERER: Mutex<Renderer> = Mutex::new(Renderer::new());
// }

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() {
    console::log_1(&"[wasm] Initialized".into());
    utils::set_panic_hook();
}
