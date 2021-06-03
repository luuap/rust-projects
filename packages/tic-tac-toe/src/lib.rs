use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum BoardValues {
  Empty = 0,
  Nought,
  Cross,
}

impl fmt::Display for BoardValues {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      BoardValues::Nought => write!(f, "{}", "O"),
      BoardValues::Cross => write!(f, "{}", "X"),
      BoardValues::Empty => write!(f, "{}", " "),
    }
  }
}

#[derive(Copy, Clone, Debug)]
#[repr(u32)]
pub enum WinningPattern {
  Row0 = 0b__111_000_000,
  Row1 = 0b__000_111_000,
  Row2 = 0b__000_000_111,
  Col0 = 0b__100_100_100,
  Col1 = 0b__010_010_010,
  Col2 = 0b__001_001_001,
  Dia0 = 0b__100_010_001, // top-left to bottom-right,
  Dia1 = 0b__001_010_100, // bottom-left to top-right,
}

impl WinningPattern {

  /**
   * returns the two points (x, y) that connects a winning pattern on the board 
   */
  pub fn points(&self) -> ((u32, u32), (u32, u32)) {
    match *self {
      Self::Row0 => ((0, 0), (2, 0)),
      Self::Row1 => ((0, 1), (2, 1)),
      Self::Row2 => ((0, 2), (2, 2)),
      Self::Col0 => ((0, 0), (0, 2)),
      Self::Col1 => ((1, 0), (1, 2)),
      Self::Col2 => ((2, 0), (2, 2)),
      Self::Dia0 => ((0, 0), (2, 2)),
      Self::Dia1 => ((0, 2), (2, 0)),
    }
  }
}

#[derive(Copy, Clone, Debug)]
pub enum GameState {
  NoughtWins(WinningPattern),
  CrossWins(WinningPattern),
  Draw,
  InProgress,
}

#[derive(Copy, Clone, Debug)]
pub enum PlayerType {
  Nought,
  Cross,
}



pub type TicTacToeBoard =  [BoardValues; 9];

impl PlayerType {
  pub fn switch_player(&self) -> Self {
    match self {
      Self::Cross => Self::Nought,
      Self::Nought => Self::Cross,
    }
  }

}

#[derive(Copy, Clone)]
pub struct TicTacToe {
  noughts: u32,
  crosses: u32,
}

impl TicTacToe {

  pub const WINNING_PATTERNS: [WinningPattern; 8] = [
    WinningPattern::Row0,
    WinningPattern::Row1,
    WinningPattern::Row2,
    WinningPattern::Col0,
    WinningPattern::Col1,
    WinningPattern::Col2,
    WinningPattern::Dia0,
    WinningPattern::Dia1,
  ];

  pub const INITIAL_MASK: u32 = 0b__100_000_000;

  pub const FINISHED_GAME_MASK: u32 = 0b__111_111_111;

  pub fn new() -> Self {
    Self {
      noughts: 0,
      crosses: 0,
    }
  }

  pub fn from_state(noughts: u32, crosses: u32) -> Self {
    Self {
      noughts,
      crosses,
    }
  }

  pub fn noughts(&self) -> u32 {
    self.noughts
  }

  pub fn crosses(&self) -> u32 {
    self.crosses
  }

  pub fn set_nought(&mut self, index: u32) {
    self.noughts |= Self::INITIAL_MASK >> index;
  }

  pub fn set_cross(&mut self, index: u32) {
    self.crosses |= Self::INITIAL_MASK >> index;
  }

  pub fn is_valid_move(&self, index: u32) -> bool {
    let mask = TicTacToe::INITIAL_MASK >> index;
    if (self.noughts | self.crosses) & mask == 0 {
      true
    } else {
      false
    }
  }

  pub fn state(&self) -> GameState {

    // Check if there is a winner
    // Note: returns early
    for &p_enum in Self::WINNING_PATTERNS.iter() {
      let p = p_enum as u32;
      if (self.noughts & p) == p {
        return GameState::NoughtWins(p_enum);
      } else if (self.crosses & p) == p {
        return GameState::CrossWins(p_enum);
      }
    };

    // If we did not return early, then there is no winner,
    // but still check if game is finised to check for a draw,
    // if not, then game is still in progress
    // Note: when game is finished, the bits of noughts and of crosses should be complements
    if (self.noughts ^ self.crosses) == Self::FINISHED_GAME_MASK {
      GameState::Draw
    } else {
      GameState::InProgress
    }

  }

}

pub struct TicTacToeGame {
  current_player: PlayerType,
  game: TicTacToe,
  is_vs_ai: bool,
}

impl TicTacToeGame {

  pub fn new(initial_player: PlayerType, is_vs_ai: bool) -> Self {
    Self {
      current_player: initial_player,
      game: TicTacToe::new(),
      is_vs_ai,
    }
  }

  pub fn continue_from(game: TicTacToe, current_player: PlayerType, is_vs_ai: bool) -> Self {
    Self {
      current_player,
      game,
      is_vs_ai,
    }
  }

  pub fn do_move(&mut self, index: u32) {

    if !self.game.is_valid_move(index) { return };

    match self.current_player {
      PlayerType::Cross => self.game.set_cross(index),
      PlayerType::Nought => self.game.set_nought(index),
    };
    self.current_player = self.current_player.switch_player();

    if matches!(self.game.state(), GameState::InProgress) && self.is_vs_ai {
      let best_move = TicTacToeAI::find_best_move(self.current_player, self.game);
      match self.current_player {
        PlayerType::Cross => self.game.set_cross(best_move),
        PlayerType::Nought => self.game.set_nought(best_move),
      };
      self.current_player = self.current_player.switch_player();
    }

  }

  pub fn board(&self) -> TicTacToeBoard {

    let mut board: TicTacToeBoard = [BoardValues::Empty; 9];
    let mut mask: u32 = TicTacToe::INITIAL_MASK;

    for board_value in board.iter_mut() {

      let is_nought = self.game.noughts() & mask;
      let is_cross  = self.game.crosses() & mask;

      if is_nought != 0 {
        *board_value = BoardValues::Nought
      } else if is_cross != 0 {
        *board_value = BoardValues::Cross
      }

      mask >>= 1;

    };

    board
  }

  // delegate functions

  pub fn state(&self) -> GameState {
    self.game.state()
  }

}

pub struct TicTacToeAI {}

impl TicTacToeAI {

  pub fn find_best_move(current_player: PlayerType, game: TicTacToe) -> u32 {

    let board = game.noughts() | game.crosses();
    let mut mask = TicTacToe::INITIAL_MASK;
    let mut best_move: Option<u32> = None;

    match current_player {
      PlayerType::Nought => {
        let mut best_score = std::i32::MAX;
        for i in 0..9 {
          let mut next_game = game;
          next_game.set_nought(i);
          if (board & mask) == 0 {
            let next_player = current_player.switch_player();
            // alpha is initially set as the worst score for maximizer, beta is set to the worst score for minimizer
            let new_score = Self::minimax(next_player, next_game, std::i32::MIN, std::i32::MAX);
            if new_score < best_score {
              best_score = new_score;
              best_move = Some(i);
            }
          }
          mask >>= 1;
        }
      },
      PlayerType::Cross => {
        let mut best_score = std::i32::MIN;
        for i in 0..9 {
          let mut next_game = game;
          next_game.set_nought(i);
          if (board & mask) == 0 {
            let next_player = current_player.switch_player();
            let new_score = Self::minimax(next_player, next_game, std::i32::MIN, std::i32::MAX);
            if new_score > best_score {
              best_score = new_score;
              best_move = Some(i);
            }
          }
          mask >>= 1;
        }
      },
    }

    if let Some(best_move) = best_move {
      best_move as u32
    } else {
      panic!("No moves left, reached invalid game state")
    }
  }

  pub fn minimax(current_player: PlayerType, game: TicTacToe, alpha: i32, beta: i32) -> i32 {
    use std::cmp::{min, max};
    match game.state() {
      GameState::NoughtWins(_) => -1,
      GameState::CrossWins(_) => 1,
      GameState::Draw => 0,
      GameState::InProgress => {

        let board = game.noughts() | game.crosses();
        let mut mask = TicTacToe::INITIAL_MASK;

        let next_player = current_player.switch_player();
        match current_player {
          // minimizer behaviour
          PlayerType::Nought => {
            // set to worst case
            let mut best_score = std::i32::MAX;

            // for each possible move
            for i in 0..9 {
              
              // Make a copy of the game and update
              let mut next_game = game;
              next_game.set_nought(i);

              // check if index is available
              if (board & mask) == 0 {

                // minimize best score
                // Note: pass down best_score as beta
                best_score = min(best_score, Self::minimax(next_player, next_game, alpha, best_score));

                // If the best score for this node (minimizer) is less than or equal to alpha (the best value among previously searched siblings for the parent (maximizer)), stop searching the rest of the children.
                // Once best_score is less than or equal to alpha, it is guaranteed that the parent (maximizer) will never choose the value of this node, 
                // because this node (minimizer) will never choose anything higher than the current best_score (reacll that higher is better for parent).
                if best_score <= alpha {
                  break
                }
              }

              mask >>= 1;

            }
            best_score
          },

          // do the same but for maximizer
          PlayerType::Cross => {
            let mut best_score = std::i32::MIN;
            for i in 0..9 {
              let mut next_game = game;
              next_game.set_cross(i);
              if (board & mask) == 0 {

                // maximize the best_score
                // Note: pass down best_score as alpha
                best_score = max(best_score, Self::minimax(next_player, next_game, best_score, beta));

                // If the best score for this node (maximizer) is greater than or equal to beta (the best value among previously searched siblings for the parent (minimizer)), stop searching the rest of the children.
                // Once best_score is greater than or equal to beta, it is guaranteed that the parent (minimizer) will never choose the value of this node, 
                // because this node (maximizer) will never choose anything lower than the current best_score (recall that lower is better for parent).
                if best_score >= beta {
                  break
                }

              }
              mask >>= 1;
            }
            best_score
          },
        }
      }
    }
  }
}
