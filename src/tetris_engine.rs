use getrandom;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::{CanvasRenderingContext2d, OffscreenCanvas, Performance};
use crate::tetris::{Tetris, TetrisBuilder, Randomizer};

const PLAYFIELD_DIM: (usize, usize) = (10, 20); // width, height

#[wasm_bindgen]
pub struct TetrisEngine {
  timer: Performance,
  ctx: CanvasRenderingContext2d, // TODO: must be OffscreenCanvasRenderingContext2d, but it's not added in yet
  canvas: OffscreenCanvas,
  square: Square,
  tetris: Tetris, // tetris logic and state
  last_update_time: f64,
}

#[wasm_bindgen]
impl TetrisEngine {
  #[wasm_bindgen(constructor)]
  pub fn new(canvas: OffscreenCanvas, options: &JsValue) -> Self {
    console::log_1(&"[Tetris] Started game".into(),);
    let ctx = canvas
      .get_context_with_context_options("2d", options)
      .expect("Failed to get context")
      .unwrap()
      .dyn_into::<CanvasRenderingContext2d>()
      .unwrap();

    canvas.set_width(canvas.height() / 2);

    let square = {
      let square_length = canvas.height() as f64 / PLAYFIELD_DIM.1 as f64;
      let square_padding = (square_length * 0.1_f64).sqrt();
      Square::new(square_length, square_padding)
    };

    // get timer from global scope
    let timer = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("performance"))
      .expect("failed to get performance from global object")
      .dyn_into::<web_sys::Performance>()
      .unwrap();

    let last_update_time = timer.now();

    let randomizer: Box<dyn Randomizer<u32>> = {
      
      struct WebRandomizer;
      impl Randomizer<u32> for WebRandomizer {
          fn get_random(&mut self) -> u32 {
            let mut buf = [0u8; 1];
            getrandom::getrandom(&mut buf).unwrap();
            buf[0] as u32
          }
      }

      Box::new(WebRandomizer {})
    };

    let tetris = TetrisBuilder {
      width: PLAYFIELD_DIM.0,
      height: PLAYFIELD_DIM.1,
      randomizer,
    }.build();

    Self {
      timer,
      ctx,
      canvas,
      square,
      tetris,
      last_update_time,
    }
  }

  pub fn render(&self) {

    // draw object
    for col in self.tetris.curr_block.pos.column_iter() {
      self
        .square
        .draw(&self.ctx, (col[(0, 0)] as f64, col[(1, 0)] as f64), self.match_color(self.tetris.curr_block.block_type as u32));
    }

    // draw playfield
    for (i, p) in self.tetris.playfield.borrow().iter().enumerate() {
      if *p > 0 {
        self.square.draw(
          &self.ctx,
          ((i % PLAYFIELD_DIM.0) as f64, (i as f64 / PLAYFIELD_DIM.0 as f64).floor()),
          self.match_color(*p),
        );
      }
    }
  }

  fn match_color(&self, n: u32) -> &'static str {
    match n {
      1 => "red",
      2 => "orange",
      3 => "yellow",
      4 => "green",
      5 => "blue",
      6 => "indigo",
      7 => "pink",
      _ => "black",
    }
  }

  pub fn clear(&self) {
    self.ctx.clear_rect(
      0_f64,
      0_f64,
      self.canvas.width() as f64,
      self.canvas.height() as f64,
    );
  }

  pub fn update(&mut self) {
    let now = self.timer.now();
    if now > self.last_update_time + 1000_f64 {
      self.last_update_time = now;
      self.tetris.update();
    }
  }

  #[wasm_bindgen(js_name = handleKeyboardInput)]
  pub fn handle_keyboard_input(&mut self, key: &str) {
    // TODO: do keymappings with enums
    self.tetris.do_action(key);
  }

  pub fn resize(&mut self, _width: u32, height: u32) {
    self.canvas.set_width(height / 2);
    self.canvas.set_height(height);
    self.square = {
      let square_length = self.canvas.height() as f64 / PLAYFIELD_DIM.1 as f64;
      let square_padding = (square_length * 0.1_f64).sqrt();
      Square::new(square_length, square_padding)
    };
    self.render();
  }

}

pub struct Square {
  length: f64,
  padding: f64,
}

impl Square {
  pub fn new(length: f64, padding: f64) -> Self {
    Self { length, padding }
  }

  // TODO create a trait for this
  pub fn draw(&self, ctx: &CanvasRenderingContext2d, pos: (f64, f64), color: &str) {
    let (l, p) = (self.length, self.padding);
    let (x, y) = (pos.0 * l, pos.1 * l);
    ctx.set_fill_style(&JsValue::from(color));
    ctx.fill_rect(x + p, y + p, l - (p * 2_f64), l - (p * 2_f64));
  }
}
