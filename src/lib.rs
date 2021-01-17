mod utils;
mod sliding_square;
mod tetris;

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

// #[wasm_bindgen]
// pub struct SlidingSquare {
//     ctx: CanvasRenderingContext2d, // must be OffscreenCanvasRenderingContext2d, but it's not added in yet
//     canvas: OffscreenCanvas,
//     obj: Square,
// }

// #[wasm_bindgen]
// impl SlidingSquare {

//     #[wasm_bindgen(constructor)]
//     pub fn new(canvas: OffscreenCanvas, options: &JsValue) -> Self {
//         let ctx = canvas
//             .get_context_with_context_options("2d", options)
//             .unwrap()
//             .unwrap()
//             .dyn_into::<CanvasRenderingContext2d>()
//             .unwrap();
//         Self {
//             ctx,
//             canvas,
//             obj: Square::new(),
//         }
//     }

//     pub fn render(&self) {
//         self.obj.draw(&self.ctx);
//     }

//     pub fn clear(&self) {
//         self.ctx.clear_rect(0., 0., self.canvas.width() as f64, self.canvas.height() as f64);
//     }

//     #[wasm_bindgen(js_name = handleKeyboardInput)]
//     pub fn handle_keyboard_input(&mut self, key: &str) {
//         // TODO: do keymappings
//         self.obj.r#move(&self.ctx, key);
//     }
// }

// pub struct Square {
//     // if we have have borrowed context here, then we need to add lifetimes annotations to this struct and the on struct that owns it
//     // so we need to annotate Moving Square as well, however, wasm_bindgen does not support yet annotations
//     color: String,
//     x: f64,
//     y: f64,
//     length: f64,
//     velocity: f64,
// }

// impl Square {
//     pub fn new() -> Self {
//         Self {
//             color: String::from("red"),
//             x: 0.,
//             y: 0.,
//             length: 30.,
//             velocity: 30.,
//         }
//     }

//     pub fn r#move(&mut self, ctx: &CanvasRenderingContext2d, dir: &str) {
//         match dir {
//             "w" => self.y = (self.y - self.velocity).max(0.),
//             "s" => self.y = (self.y + self.velocity).min(ctx.canvas().unwrap().height() as f64 - self.length),
//             "a" => self.x = (self.x - self.velocity).max(0.),
//             "d" => self.x = (self.x + self.velocity).min(ctx.canvas().unwrap().width() as f64 - self.length),
//             _ => ()
//         }
//     }

//     pub fn draw(&self, ctx: &CanvasRenderingContext2d ) {
//         ctx.set_fill_style(&JsValue::from(&self.color));
//         ctx.fill_rect(self.x, self.y, self.length, self.length);
//     }
// }
