use tetris::{Tetris, TetrisBuilder, Randomizer, TetrisAction, MoveDirection, RotationDirection};
use rand::{Rng, rngs::ThreadRng};

trait Renderable {
  fn render(&self);
}

impl Renderable for Tetris {
  fn render(&self) {
    let mut playfield = self.playfield.borrow().clone();

    for _ in 0..self.dim.width + 2 {
      print!("-");
    }

    println!("");
    print!("|");

    for col in self.curr_block.pos.column_iter() {
      playfield[col[(1, 0)] as usize * self.dim.width + col[(0, 0)] as usize] = self.curr_block.block_type as u32;
    }

    for (i, p) in playfield.iter().enumerate() {

      let c = match *p {
        1 => 'I',
        2 => 'J',
        3 => 'L',
        4 => 'O',
        5 => 'S',
        6 => 'T',
        7 => 'Z',
        _ => 'x',
      };

      if i != 0 && i % self.dim.width == 0 {
        println!("|");
        print!("|");
      }
      print!("{}", c);
    }

    println!("|");
    for _ in 0..self.dim.width + 2 {
      print!("-");
    }
  }
}

fn main() {
  println!("Hello, world!");

  struct MyRandomizer {
      rng: ThreadRng,
  }

  impl Randomizer<u32> for MyRandomizer {
      fn get_random(&mut self) -> u32 {
          self.rng.gen_range(1..=7)
      }
  }

  let mut tetris = TetrisBuilder {
    width: 10,
    height: 20,
    randomizer: Box::new(MyRandomizer { 
      rng: rand::thread_rng()
    }),
  }.build();

  use MoveDirection::*;
  use RotationDirection::*;
  use TetrisAction::*;

  tetris.do_action(Move(Down));
  tetris.do_action(Move(Down));
  tetris.do_action(Move(Down));
  tetris.do_action(Move(Right));
  tetris.do_action(Move(Right));
  tetris.do_action(Rotate(CounterClockwise));
  tetris.do_action(Move(Right));
  tetris.do_action(Move(Right));
  tetris.do_action(Move(Right));
  tetris.render();
}
