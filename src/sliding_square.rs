use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use js_sys::{Uint32Array};
use web_sys::{CanvasRenderingContext2d, OffscreenCanvas};

#[derive(Copy, Clone)]
pub enum MoveDirection {
  Up = 0,
  Down,
  Left,
  Right,
}

#[wasm_bindgen]
pub struct SlidingSquare {
  ctx: CanvasRenderingContext2d, // must be OffscreenCanvasRenderingContext2d, but it's not added in yet
  canvas: OffscreenCanvas,
  obj: Square,
}

#[wasm_bindgen]
impl SlidingSquare {
  #[wasm_bindgen(constructor)]
  pub fn new(canvas: OffscreenCanvas, options: &JsValue) -> Self {
    let ctx = canvas
      .get_context_with_context_options("2d", options)
      .unwrap()
      .unwrap()
      .dyn_into::<CanvasRenderingContext2d>()
      .unwrap();

    Self {
      ctx,
      obj: Square::new(&canvas),
      canvas,
    }
  }

  pub fn update(&self) {
    ()
  }

  pub fn render(&self) {
    self.obj.draw(&self.ctx);
  }

  pub fn clear(&self) {
    self.ctx.clear_rect(
      0.,
      0.,
      self.canvas.width() as f64,
      self.canvas.height() as f64,
    );
  }

  #[wasm_bindgen(js_name = handleInput)]
  pub fn handle_input(&mut self, key: i32) {
    use MoveDirection::*;
    match key {
      0 => self.obj.r#move(Up),
      1 => self.obj.r#move(Down),
      2 => self.obj.r#move(Left),
      3 => self.obj.r#move(Right),
      _ => ()
    }
  }

  pub fn resize(&mut self, width: u32, height: u32) -> Uint32Array {

    let reference_length = std::cmp::min(width, height);
    let (new_width, new_height) = (reference_length, reference_length);
    self.canvas.set_width(new_width);
    self.canvas.set_height(new_height);
    self.obj = Square::new(&self.canvas);

    self.render();
    let res: &[u32] = &[new_width, new_height];
    Uint32Array::from(res)
  }

}

pub struct Square {
  // if we have have borrowed context here, then we need to add lifetimes annotations to this struct and the on struct that owns it
  // so we need to annotate Moving Square as well, however, wasm_bindgen does not support yet annotations
  color: String,
  x: f64,
  y: f64,
  length: f64,
  bounds: (f64, f64),
}

impl Square {
  pub fn new(canvas: &OffscreenCanvas) -> Self {
    // console::log_2(&"Width:".into(), &m_coord.height.into());
    Self {
      color: String::from("red"),
      x: 0.,
      y: 0.,
      length: canvas.height() as f64 / 10.,
      bounds: (canvas.width() as f64, canvas.height() as f64),
    }
  }

  pub fn r#move(&mut self, dir: MoveDirection) {
    use MoveDirection::*;
    match dir {
      Up => self.y = (self.y - self.bounds.1 / 10.).max(0.),
      Down => self.y = (self.y + self.bounds.1 / 10.).min((self.bounds.1) - self.length),
      Left => self.x = (self.x - self.bounds.0 / 10.).max(0.),
      Right => self.x = (self.x + self.bounds.0 / 10.).min((self.bounds.0) - self.length)
    }
  }

  pub fn draw(&self, ctx: &CanvasRenderingContext2d) {
    ctx.set_fill_style(&JsValue::from(&self.color));
    ctx.fill_rect(self.x, self.y, self.length, self.length);
  }
}
