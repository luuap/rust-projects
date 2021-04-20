use std::cell::RefCell;

use nalgebra as na;
use na::{Matrix3x4, Matrix3};

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

#[derive(Copy, Clone)]
pub enum MoveDirection {
  Down,
  Left,
  Right,
}

#[derive(Copy, Clone)]
pub enum RotationDirection {
  Clockwise,
  CounterClockwise,
}

pub enum TetrisAction {
  Move(MoveDirection),
  Rotate(RotationDirection),
  // TODO drop
}

pub trait Randomizer<T> {
  fn get_random(&mut self) -> T;
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

impl BlockType {
  fn get_inital(&self) -> (Matrix3x4<isize>, usize) { // the last value is the ith column in the matrix, pivot index
    // blocks are initially shifted 3 to the right TODO: base the offset on width instead of hardcoding
    match *self {
      Self::I => (Matrix3x4::new(3, 4, 5, 6,
                                 2, 2, 2, 2,
                                 1, 1, 1, 1), 2),

      Self::J => (Matrix3x4::new(3, 4, 5, 5,
                                 1, 1, 1, 2,
                                 1, 1, 1, 1), 1),

      Self::L => (Matrix3x4::new(3, 4, 5, 5,
                                 1, 1, 1, 0,
                                 1, 1, 1, 1), 1),

      Self::O => (Matrix3x4::new(4, 4, 5, 5,
                                 1, 2, 1, 2,
                                 1, 1, 1, 1), 0),

      Self::S => (Matrix3x4::new(4, 5, 3, 4,
                                 1, 1, 2, 2,
                                 1, 1, 1, 1), 0),

      Self::T => (Matrix3x4::new(3, 4, 4, 5,
                                 1, 1, 2, 1,
                                 1, 1, 1, 1), 1),

      Self::Z => (Matrix3x4::new(3, 4, 4, 5,
                                 1, 1, 2, 2,
                                 1, 1, 1, 1), 1),
    }
  }

  fn get_random(rng: &mut Box<dyn Randomizer<u32>>) -> Self {
    match rng.get_random() % 7 {
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
  // pub color: &'static str,
  pub rot_state: RotationState,
  pub block_type: BlockType,
  pub pos: Matrix3x4<isize>,
  pub pivot_idx: usize,
}

impl Block {
  pub fn new(block_type: BlockType) -> Self {

    let (pos, pivot_idx) = block_type.get_inital();

    let rot_state = RotationState::Deg0;

    Self {
      pos,
      pivot_idx,
      // color,
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
          // I block has only two states, 0 and 90
          // if 0, rotate clockwise
          // if 90, rotate cclockwise
          (I, Deg0) => {
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
          // S and z blocks have only two states, 0 and 270
          // if 0, rotate cclockwise
          // if 270, rotate clockwise 
          (S, Deg0) | (Z, Deg0)=> {
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
        let (pivot_x, pivot_y) = (current_pos[(0, self.pivot_idx)], current_pos[(1, self.pivot_idx)]);

        use RotationDirection::*;
        let (r_a, r_b) = match dir { 
          Clockwise => (-1, 1),
          CounterClockwise => (1, -1),
        };

        let t_1 = Matrix3::new( 1, 0, pivot_x,
                                0, 1, pivot_y,
                                0, 0, 1 );

        let t_2 = Matrix3::new( 1, 0, -pivot_x,
                                0, 1, -pivot_y,
                                0, 0, 1 );

        let rot = Matrix3::new( 0,    r_a, 0,
                                r_b,  0,   0,
                                0,    0,   1);

        (t_1 * rot * t_2 * current_pos, next_rot_state)
      }
    }
  }

  pub fn try_move(&self, dir: MoveDirection) -> Matrix3x4<isize> {
    let pos = self.pos.clone();

    use MoveDirection::*;
    let (d_x, d_y) = match dir {
      Down => (0, 1),
      Left => (-1, 0),
      Right => (1, 0),
    };

    let t = Matrix3::new(1, 0, d_x,
                         0, 1, d_y,
                         0, 0, 1 );

    t * pos
  }

}

pub struct Dimensions {
  pub width: usize,
  pub height: usize,
}

pub struct TetrisBuilder {
  pub width: usize,
  pub height: usize,
  pub randomizer: Box<dyn Randomizer<u32>>,
}

impl TetrisBuilder {
  pub fn build(self) -> Tetris {
    let dim = Dimensions {
      width: self.width,
      height: self.height,
    };
    let playfield = RefCell::new(vec![0; dim.width * dim.height]);
    let mut randomizer = self.randomizer;
    let curr_block = Block::new(BlockType::get_random(&mut randomizer));
    Tetris {
      randomizer,
      dim,
      curr_block,
      playfield,
    }
  }
}

/**
 * Tetris logic
 */
pub struct Tetris {
  pub playfield: RefCell<Vec<u32>>,
  pub curr_block: Block,
  pub dim: Dimensions,
  randomizer: Box<dyn Randomizer<u32>>,
}

impl Tetris {

  pub fn update(&mut self) {
    if self.move_obj(MoveDirection::Down).is_err() {
      // add the obj in playfield
      let block_type = self.curr_block.block_type as u32;
      self
        .curr_block
        .pos
        .column_iter()
        .for_each(|col| self.playfield.borrow_mut()[self.dim.width * col[(1,0)] as usize + col[(0,0)] as usize] = block_type);

      // clear lines if any
      let copy_map: Vec<(usize, usize)> = self.playfield.borrow_mut()
        .chunks_exact(self.dim.width) // get each row
        .map(|chunk| chunk.iter().any(|&val| val == 0)) // find which rows will stay
        .enumerate() // get row numbers
        .filter_map(|(i, line)| if line { Some(i) } else { None }) // filter out the row numbers that will stay (remove the gaps between the rows)
        .rev() // reverse because we are starting from the last row
        .scan(self.dim.height, |acc, i| {
          // pair the row numbers that will stay with the bottom n rows (collapse the rows to the bottom)
          *acc -= 1;
          Some((i, *acc))
        })
        .collect();

      // TODO clumps, cascade
      copy_map.iter().for_each(|&(i, j)| {
        if i != j { // if line is going to drop
          let src_idx = i * self.dim.width;
          let dest_idx = j * self.dim.width;
          self.playfield.borrow_mut().copy_within(src_idx..(src_idx + self.dim.width), dest_idx); // copy over the row that will be dropping
          self.playfield.borrow_mut()[src_idx..dest_idx].iter_mut().for_each(|i| *i = 0); // clear the line that was moved
        }
      });

      // try to spawn in a new obj
      self.curr_block = Block::new(BlockType::get_random(&mut self.randomizer));
      if !self.not_colliding(&self.curr_block.pos) {
          // clear the board
          self.playfield.borrow_mut().iter_mut().for_each(|i| *i = 0);
      };
    };
  }

  pub fn do_action(&mut self, key: &str) {
    use MoveDirection::*;
    use RotationDirection::*;
    use TetrisAction::*;

    let action = match key {
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
    let (next_pos, next_rot_state) = self.curr_block.try_rotate(dir);
    if self.within_bounds(&next_pos) && self.not_colliding(&next_pos) {
      self.curr_block.pos = next_pos;
      self.curr_block.rot_state = next_rot_state;
    };
  }

  fn move_obj(&mut self, dir: MoveDirection) -> Result<(), ()> { // return a result because we are using this method to check if block will freeze
    let next_pos = self.curr_block.try_move(dir);
    if self.within_bounds(&next_pos) && self.not_colliding(&next_pos) {
      self.curr_block.pos = next_pos;
      Ok(())
    } else {
      Err(())
    }
  }

  fn not_colliding(&self, pos: &Matrix3x4<isize>) -> bool {
    pos
      .column_iter()
      .all(|col| self.playfield.borrow()[self.dim.width * col[(1, 0)] as usize + col[(0, 0)] as usize] == 0)
  }

  fn within_bounds(&self, pos: &Matrix3x4<isize>) -> bool {
    pos
      .column_iter()
      .all(|col| {
        col[(1, 0)] < self.dim.height as isize &&
        col[(0, 0)] >= 0 &&
        col[(0, 0)] < self.dim.width as isize
      })
  }
}