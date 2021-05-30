use tic_tac_toe::{ TicTacToe, TicTacToeGame, PlayerType, GameState };
use std::io::{ stdin, stdout, Write};

trait PrintBoard {
  fn print_board(&self);
}

impl PrintBoard for TicTacToeGame {

  fn print_board(&self) {

    let board = self.board();
    for i in (0..9).step_by(3) {
      println!("{}{}{}", board[i], board[i + 1], board[i + 2]);
    }

  }

}

fn main() {
  let state = TicTacToe::from_state(0b__000_000_000, 0b__000_000_000);
  let mut game = TicTacToeGame::continue_from(state, PlayerType::Cross, true);

  loop {

    game.print_board();

    let mut input = String::new();

    print!("Enter your next move: ");
    stdout().flush().unwrap();

    stdin()
      .read_line(&mut input)
      .expect("Failed to read line");

    let index: u32 = match input.trim().parse() {
      Ok(num) => {
        if num < 9 {
          num
        } else {
          println!("Must be a number from 0..=9, try Again");
          continue;
        }
      },
      Err(_) => {
        println!("Not a number, try Again");
        continue;
      },
    };

    game.do_move(index);

    match game.state() {
      GameState::NoughtWins(_) => {
        println!("Nought Wins");
      },
      GameState::CrossWins(_) => {
        println!("Cross Wins");
      },
      GameState::Draw => {
        println!("Draw");
      }
      GameState::InProgress => continue
    }

    game.print_board();
    game = TicTacToeGame::new(PlayerType::Cross, true);
    println!("Game is reset");
  }
}
