use crate::utils::{Vec2, Vec3};

use getrandom;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::{CanvasRenderingContext2d, OffscreenCanvas, Performance};

const PLAYFIELD_DIM: (usize, usize) = (10, 20);

pub enum MoveDirection {
  Up,
  Down,
  Left,
  Right,
}

pub enum RotationDirection {
  Clockwise,
  CounterClockwise,
}

pub enum TetrisActions {
  Move(MoveDirection),
  Rotate(RotationDirection),
  // TODO drop
}

// TODO: find a pattern to wrap js methods/objects separately
#[wasm_bindgen]
pub struct Tetris {
  timer: Performance,
  ctx: CanvasRenderingContext2d, // must be OffscreenCanvasRenderingContext2d, but it's not added in yet
  canvas: OffscreenCanvas,
  square: Square,
  obj: Block,
  playfield: Vec<u32>,
  last_update_time: f64,
}

#[wasm_bindgen]
impl Tetris {
  #[wasm_bindgen(constructor)]
  pub fn new(canvas: OffscreenCanvas, options: &JsValue) -> Self {
    let ctx = canvas
      .get_context_with_context_options("2d", options)
      .expect("Failed to get context")
      .unwrap()
      .dyn_into::<CanvasRenderingContext2d>()
      .unwrap();

    // let bounds = Vec2(height / 2_f64, height); // width is based on height
    // let velocity = Vec2(bounds.0 / PLAYFIELD_DIM.0 as f64, bounds.1 / PLAYFIELD_DIM.1 as f64);

    let square = {
      let square_length = canvas.height() as f64 / PLAYFIELD_DIM.1 as f64;
      let square_padding = (square_length * 0.1_f64).sqrt();
      Square::new(square_length, square_padding)
    };
    let obj = Block::new(BlockType::O);
    let playfield = vec![0; PLAYFIELD_DIM.0 * PLAYFIELD_DIM.1];

    // get timer from global scope
    let timer = js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("performance"))
      .expect("failed to get performance from global object")
      .dyn_into::<web_sys::Performance>()
      .unwrap();

    let last_update_time = timer.now();

    Self {
      ctx,
      square,
      obj,
      canvas,
      playfield,
      timer,
      last_update_time,
    }
  }

  pub fn render(&self) {
    // draw object
    for p in &self.obj.pos {
      self
        .square
        .draw(&self.ctx, Vec2(p.0 as f64, p.1 as f64), &self.obj.color);
    }

    // draw playfield
    for (i, p) in self.playfield.iter().enumerate() {
      if *p > 0 {
        let color = match *p {
          1 => "red",
          2 => "orange",
          3 => "yellow",
          4 => "green",
          5 => "blue",
          6 => "indigo",
          7 => "pink",
          _ => "black",
        };
        self.square.draw(
          &self.ctx,
          Vec2((i % 10) as f64, (i as f64 / PLAYFIELD_DIM.0 as f64).floor()),
          color,
        );
      }
    }
  }

  pub fn clear(&self) {
    // TODO: just clear the obj
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
      if self.move_obj(MoveDirection::Down).is_err() {
        let playfield = &mut self.playfield;

        // add the obj in playfield
        let block_type = self.obj.block_type as u32;
        self
          .obj
          .pos
          .iter()
          .for_each(|p| playfield[PLAYFIELD_DIM.0 * p.1 as usize + p.0 as usize] = block_type);

        // clear lines if any

        // find lines
        let copy_map: Vec<(usize, usize)> = playfield
          .chunks_exact(PLAYFIELD_DIM.0) // get each row
          .map(|chunk| chunk.iter().any(|&val| val == 0)) // find which rows will stay
          .enumerate() // get row numbers
          .filter_map(|(i, line)| if line { Some(i) } else { None }) // filter out the row numbers that will stay (remove the gaps between the rows)
          .rev() // reverse because we are starting from the last row
          .scan(PLAYFIELD_DIM.1, |acc, i| {
            // pair the row numbers that will stay with the bottom n rows (collapse the rows to the bottom)
            *acc -= 1;
            Some((i, *acc))
          })
          .collect();

        // TODO clumps, cascade
        copy_map.iter().for_each(|&(i, j)| {
          // console::log_3(
          //   &"Logging arbitrary values looks like".into(),
          //   &JsValue::from(i as u32),
          //   &JsValue::from(j as u32),
          // );
          if i != j {
            // line stays where it is
            let src_idx = i * PLAYFIELD_DIM.0;
            let dest_idx = j * PLAYFIELD_DIM.0;
            playfield.copy_within(src_idx..(src_idx + PLAYFIELD_DIM.0), dest_idx); // copy over the row that will be dropping
            for i in &mut playfield[src_idx..dest_idx] {
              *i = 0
            } // zero out the
          }
        });

        // try to spawn in a new obj (check if spawn area is cleared)
        let block_type = BlockType::get_random();
        self.obj = Block::new(block_type);
      };
    }
  }

  #[wasm_bindgen(js_name = handleKeyboardInput)]
  pub fn handle_keyboard_input(&mut self, key: &str) {
    // TODO: do keymappings
    use MoveDirection::*;
    use RotationDirection::*;
    use TetrisActions::*;

    let action = match key {
      "w" => Some(Move(Up)),
      "s" => Some(Move(Down)),
      "a" => Some(Move(Left)),
      "d" => Some(Move(Right)),
      "q" => Some(Rotate(CounterClockwise)),
      "e" => Some(Rotate(Clockwise)),
      _ => None,
    };

    if let Some(action) = action {
      match action {
        Move(dir) => {
          let _ = self.move_obj(dir).ok();
        }
        Rotate(dir) => self.rotate_obj(&dir),
      }
    };
  }

  fn rotate_obj(&mut self, dir: &RotationDirection) {
    // check if we can rotate
  }

  fn move_obj(&mut self, dir: MoveDirection) -> Result<(), ()> {
    // return a result because we are using this method to check if block will freeze
    let next_pos = self.obj.try_move(&dir);
    // check if in bounds
    use MoveDirection::*;
    let within_bounds: bool = match dir {
      Up => false,
      Down => next_pos
        .iter()
        .all(|&Vec2(_, y)| y < PLAYFIELD_DIM.1 as isize),
      Left => next_pos.iter().all(|&Vec2(x, _)| x >= 0),
      Right => next_pos
        .iter()
        .all(|&Vec2(x, _)| x < PLAYFIELD_DIM.0 as isize),
    };

    // check if block is going to freeze
    let can_move: bool = within_bounds
      && next_pos
        .iter()
        .all(|&Vec2(x, y)| self.playfield[PLAYFIELD_DIM.0 * y as usize + x as usize] == 0);

    if can_move {
      self.obj.set_pos(next_pos);
      Ok(())
    } else {
      Err(())
    }
  }
}

pub trait Randomizable {
  fn get_random() -> Self;
}

#[derive(Copy, Clone)]
pub enum BlockType {
  I = 1,
  J,
  L,
  O,
  S,
  T,
  Z,
}

impl Randomizable for BlockType {
  fn get_random() -> Self {
    let mut buf = [0u8; 1];
    getrandom::getrandom(&mut buf).unwrap();
    match buf[0] % 7 {
      0 => Self::I,
      1 => Self::J,
      2 => Self::L,
      3 => Self::O,
      4 => Self::S,
      5 => Self::T,
      _ => Self::Z,
    }
  }
}

pub struct Block {
  color: &'static str,
  pub block_type: BlockType,
  pub pos: [Vec2<isize>; 4],
}

impl Block {
  pub fn new(block_type: BlockType) -> Self {
    use BlockType::*;
    let (pos, color) = match block_type {
      // TODO random rotation
      I => ([Vec2(2, 0), Vec2(2, 1), Vec2(2, 2), Vec2(2, 3)], "red"),
      J => ([Vec2(0, 1), Vec2(1, 1), Vec2(2, 1), Vec2(2, 2)], "orange"),
      L => ([Vec2(0, 1), Vec2(1, 1), Vec2(2, 1), Vec2(2, 0)], "yellow"),
      O => ([Vec2(1, 1), Vec2(1, 2), Vec2(2, 1), Vec2(2, 2)], "green"),
      S => ([Vec2(1, 1), Vec2(2, 1), Vec2(0, 2), Vec2(1, 2)], "blue"),
      T => ([Vec2(0, 1), Vec2(1, 1), Vec2(1, 2), Vec2(2, 1)], "indigo"),
      Z => ([Vec2(0, 1), Vec2(1, 1), Vec2(1, 2), Vec2(2, 2)], "pink"),
    };

    Self {
      pos,
      color,
      block_type,
    }
  }

  pub fn set_pos(&mut self, new_pos: [Vec2<isize>; 4]) {
    self.pos = new_pos;
  }

  pub fn try_rotate(&self, dir: &RotationDirection) -> [Vec2<isize>; 4] {
    let mut rotated_pos = self.pos.clone();

    // let clockwise_rotation_matr = [Vec3()]
    // translate to the pivot, then rotate
    [Vec2(0, 0), Vec2(1, 0), Vec2(0, 1), Vec2(1, 1)]
  }

  pub fn try_move(&self, dir: &MoveDirection) -> [Vec2<isize>; 4] {
    let mut moved_pos = self.pos.clone();

    use MoveDirection::*;
    let delta = match dir {
      Up => Vec2(0, -1),
      Down => Vec2(0, 1),
      Left => Vec2(-1, 0),
      Right => Vec2(1, 0),
    };

    for p in &mut moved_pos {
      *p += delta;
    }
    moved_pos
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
  pub fn draw(&self, ctx: &CanvasRenderingContext2d, pos: Vec2<f64>, color: &str) {
    let (l, p) = (self.length, self.padding);
    let (x, y) = (pos.0 * l, pos.1 * l);
    ctx.set_fill_style(&JsValue::from(color));
    ctx.fill_rect(x + p, y + p, l - (p * 2_f64), l - (p * 2_f64));
  }
}
