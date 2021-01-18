use getrandom;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::{CanvasRenderingContext2d, OffscreenCanvas, Performance};

use nalgebra as na;
use na::{Matrix3x4, Matrix3};

const PLAYFIELD_DIM: (usize, usize) = (10, 20);

#[derive(Copy, Clone)]
pub enum RotationState {
  Deg0,
  Deg90,
  Deg180,
  Deg270,
}

impl RotationState {
  pub fn get_next_state(&self, dir: RotationDirection) -> Self {
    use RotationDirection::*;
    match *self {
      Self::Deg0 => if let Clockwise = dir { Self::Deg90 } else { Self::Deg270 },
      Self::Deg90 => if let Clockwise = dir { Self::Deg180 } else { Self::Deg0},
      Self::Deg180 => if let Clockwise = dir { Self::Deg270 } else { Self::Deg90 },
      Self::Deg270 => if let Clockwise = dir { Self::Deg0 } else { Self::Deg180 },
    }
  }
}

pub enum MoveDirection {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Copy, Clone)]
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

    let square = {
      let square_length = canvas.height() as f64 / PLAYFIELD_DIM.1 as f64;
      let square_padding = (square_length * 0.1_f64).sqrt();
      Square::new(square_length, square_padding)
    };
    let obj = Block::new(BlockType::S);
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
    for col in self.obj.pos.column_iter() {
      self
        .square
        .draw(&self.ctx, (col[(0, 0)] as f64, col[(1, 0)] as f64), &self.obj.color);
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
          ((i % 10) as f64, (i as f64 / PLAYFIELD_DIM.0 as f64).floor()),
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
          .column_iter()
          .for_each(|col| playfield[PLAYFIELD_DIM.0 * col[(1,0)] as usize + col[(0,0)] as usize] = block_type);

        // clear lines if any
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
          if i != j { // if line is going to drop
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
        Rotate(dir) => self.rotate_obj(dir),
      }
    };
  }

  fn rotate_obj(&mut self, dir: RotationDirection) {
    let (next_pos, next_rot_state) = self.obj.try_rotate(dir);
    // TODO: check if we can rotate
    self.obj.pos= next_pos;
    self.obj.rot_state = next_rot_state;
  }

  fn move_obj(&mut self, dir: MoveDirection) -> Result<(), ()> {
    // return a result because we are using this method to check if block will freeze
    let next_pos = self.obj.try_move(&dir);
    // check if in bounds
    use MoveDirection::*;
    let within_bounds: bool = match dir {
      // TODO: puts great assumption on the last moves were valid (if we were to be in an invalid state, some checks will not be run)
      Up => false,
      Down => next_pos
        .column_iter()
        .all(|col| col[(1, 0)] < PLAYFIELD_DIM.1 as isize), // y val
      Left => next_pos
        .column_iter()
        .all(|col| col[(0, 0)] >= 0), // x val
      Right => next_pos
        .column_iter()
        .all(|col| col[(0, 0)] < PLAYFIELD_DIM.0 as isize),
    };

    // check if block is going to freeze
    let can_move: bool = within_bounds
      && next_pos
        .column_iter()
        .all(|col| self.playfield[PLAYFIELD_DIM.0 * col[(1, 0)] as usize + col[(0, 0)] as usize] == 0);

    if can_move {
      self.obj.pos = next_pos;
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
  pub rot_state: RotationState,
  pub block_type: BlockType,
  pub pos: Matrix3x4<isize>,
  // TODO: pointers pointing to pivot
}

impl Block {
  pub fn new(block_type: BlockType) -> Self {
    use BlockType::*;

    let (pos, color) = match block_type {
      // TODO random rotation
      I => (Matrix3x4::new(0, 1, 2, 3,
                           2, 2, 2, 2,
                           1, 1, 1, 1), "red"),

      J => (Matrix3x4::new(0, 1, 2, 2,
                           1, 1, 1, 2,
                           1, 1, 1, 1), "orange"),

      L => (Matrix3x4::new(0, 1, 2, 2,
                           1, 1, 1, 0,
                           1, 1, 1, 1), "yellow"),

      O => (Matrix3x4::new(1, 1, 2, 2,
                           1, 2, 1, 2,
                           1, 1, 1, 1), "green"),

      S => (Matrix3x4::new(1, 2, 0, 1,
                           1, 1, 2, 2,
                           1, 1, 1, 1), "blue"),

      T => (Matrix3x4::new(0, 1, 1, 2,
                           1, 1, 2, 1,
                           1, 1, 1, 1), "indigo"),

      Z => (Matrix3x4::new(0, 1, 1, 2,
                           1, 1, 2, 2,
                           1, 1, 1, 1), "pink"),
    };

    let rot_state = RotationState::Deg0;

    Self {
      pos,
      color,
      block_type,
      rot_state,
    }
  }

  pub fn try_rotate(&self, mut dir: RotationDirection) -> (Matrix3x4<isize>, RotationState) {
    let current_pos = self.pos.clone();

    use BlockType::*;
    match self.block_type {
      O => (current_pos, self.rot_state),
      _ => {
        let mut next_rot_state = self.rot_state;
        
        use RotationState::*;
        match (self.block_type, self.rot_state) {
          (I, Deg0) => {
            // only two states, 0 and 90
            // if 0, rotate clockwise
            // if 90, rotate cclockwise
            next_rot_state = Deg90;
            dir = Clockwise;
          },
          (I, Deg90) => {
            next_rot_state = Deg0;
            dir = CounterClockwise;
          },
          (J, _) | (L, _) | (T, _) => {
            next_rot_state = self.rot_state.get_next_state(dir);
          },
          (S, Deg0) | (Z, Deg0)=> {
            // only two states, 0 and 270
            // if 0, rotate cclockwise
            // if 270, rotate clockwise 
            next_rot_state = Deg270;
            dir = CounterClockwise;
          },
          (S, Deg270) | (Z, Deg270)=> {
            next_rot_state = Deg0;
            dir = Clockwise;
          },
          _ => ()
        };

        // get pivot
        let (pivot_x, pivot_y) = match self.block_type {
          I => (current_pos[(0, 2)], current_pos[(1, 2)]),
          J => (current_pos[(0, 1)], current_pos[(1, 1)]),
          L => (current_pos[(0, 1)], current_pos[(1, 1)]),
          O => (current_pos[(0, 0)], current_pos[(1, 0)]),
          S => (current_pos[(0, 0)], current_pos[(1, 0)]),
          T => (current_pos[(0, 1)], current_pos[(1, 1)]),
          Z => (current_pos[(0, 1)], current_pos[(1, 1)]),
        };

        use RotationDirection::*;
        let (r_a, r_b) = match dir { 
          Clockwise => (-1, 1),
          CounterClockwise => (1, -1),
        };

        // console::log_3(
        //   &"Logging arbitrary values looks like".into(),
        //   &JsValue::from(d_x as u32),
        //   &JsValue::from(d_y as u32),
        // );

        let t_1 = Matrix3::new( 1, 0, pivot_x,
                                0, 1, pivot_y,
                                0, 0, 1 );

        let t_2 = Matrix3::new( 1, 0, -pivot_x,
                                0, 1, -pivot_y,
                                0, 0, 1 );

        let rot = Matrix3::new( 0,    r_a, 0,
                                r_b,  0,   0,
                                0,    0,   1);

        // console::log_4(
        //   &JsValue::from(format!("{}", rotated_pos)),
        //   &JsValue::from(format!("{}", t_2.to_homogeneous() * rotated_pos)),
        //   &JsValue::from(format!("{}",  rot * t_2.to_homogeneous() * rotated_pos)),
        //   &JsValue::from(format!("{}", result)),

        // );
        (t_1 * rot * t_2 * current_pos, next_rot_state)
      }
    }
  }

  pub fn try_move(&self, dir: &MoveDirection) -> Matrix3x4<isize> {
    let pos = self.pos.clone();

    use MoveDirection::*;
    let (d_x, d_y) = match dir {
      Up => (0, -1),
      Down => (0, 1),
      Left => (-1, 0),
      Right => (1, 0),
    };

    let t = Matrix3::new( 1, 0, d_x,
                          0, 1, d_y,
                          0, 0, 1 );

    t * pos
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
