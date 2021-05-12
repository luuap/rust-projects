use getrandom;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::{Uint32Array};
use web_sys::console;
use web_sys::{CanvasRenderingContext2d, OffscreenCanvas, Performance};
use crate::tetris::{Tetris, TetrisBuilder, Randomizer, MoveDirection, RotationDirection, TetrisAction};

const PLAYFIELD_DIM: (usize, usize) = (10, 20); // width, height

/**
 * Input Ids
 * 0 - Down
 * 1 - Left
 * 2 - Right
 * 3 - Rotate counteclockwise
 * 4 - Rotate clockwise
 */
#[wasm_bindgen]
pub struct WebTetrisEngine {
  timer: Performance,
  ctx: CanvasRenderingContext2d, // TODO: must be OffscreenCanvasRenderingContext2d, but it's not added in yet
  canvas: OffscreenCanvas,
  square_drawer: SquareDrawer,
  tetris: Tetris, // tetris logic and state
  last_update_time: f64,
}

#[wasm_bindgen]
impl WebTetrisEngine {
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

    let square_drawer = {
      let square_length = canvas.height() as f64 / PLAYFIELD_DIM.1 as f64;
      let square_padding = (square_length * 0.1_f64).sqrt();
      SquareDrawer::new(square_length, square_padding)
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
      square_drawer,
      tetris,
      last_update_time,
    }
  }

  pub fn render(&self) {

    // draw object
    for col in self.tetris.curr_block.pos.column_iter() {
      self
        .square_drawer
        .draw(&self.ctx, (col[(0, 0)] as f64, col[(1, 0)] as f64), Self::match_color(self.tetris.curr_block.block_type as u32));
    }

    // draw playfield
    for (i, p) in self.tetris.playfield.borrow().iter().enumerate() {
      if *p > 0 {
        self
          .square_drawer
          .draw(
            &self.ctx,
            ((i % PLAYFIELD_DIM.0) as f64, (i as f64 / PLAYFIELD_DIM.0 as f64).floor()),
            Self::match_color(*p),
          );
      }
    }
  }

  fn match_color(n: u32) -> &'static str {
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

  #[wasm_bindgen(js_name = handleInput)]
  pub fn handle_input(&mut self, key: u32) {
    
    use MoveDirection::*;
    use RotationDirection::*;
    use TetrisAction::*;

    // TODO: create macro/test to ensure all variants of TetrisAction are returned
    // https://stackoverflow.com/questions/58715081/how-to-ensure-every-enum-variant-can-be-returned-from-a-specific-function-at-com

    let action = match key {
      0 => Some(Move(Down)),
      1 => Some(Move(Left)),
      2 => Some(Move(Right)),
      3 => Some(Rotate(CounterClockwise)),
      4 => Some(Rotate(Clockwise)),
      _ => None,
    };

    if let Some(action) = action {
      self.tetris.do_action(action);
    };

  }

  pub fn resize(&mut self, _width: u32, height: u32) -> Uint32Array {

    let (new_width, new_height) = (height / 2, height);
    self.canvas.set_width(new_width);
    self.canvas.set_height(new_height);
    self.square_drawer = {
      let square_length = self.canvas.height() as f64 / PLAYFIELD_DIM.1 as f64;
      let square_padding = (square_length * 0.1_f64).sqrt();
      SquareDrawer::new(square_length, square_padding)
    };
    self.render();
    let res: &[u32] = &[new_width, new_height];
    Uint32Array::from(res)
  }

}

pub struct SquareDrawer {
  length: f64,
  padding: f64,
}

impl SquareDrawer {
  pub fn new(length: f64, padding: f64) -> Self {
    Self { length, padding }
  }

  pub fn draw(&self, ctx: &CanvasRenderingContext2d, pos: (f64, f64), color: &str) {
    let (l, p) = (self.length, self.padding);
    let (x, y) = (pos.0 * l, pos.1 * l);
    ctx.set_fill_style(&JsValue::from(color));
    ctx.fill_rect(x + p, y + p, l - (p * 2_f64), l - (p * 2_f64));
  }
}
