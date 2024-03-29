mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, OffscreenCanvas};
use serde::{Serialize, Deserialize};

use tic_tac_toe::{ TicTacToeGame, PlayerType, GameState, WinningPattern };

#[derive(Serialize, Deserialize)]
pub struct Coordinates {
  pub x: f64,
  pub y: f64,
}

#[derive(Serialize, Deserialize)]
pub struct InputData {
  pub coordinates: Coordinates
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/**
 Input Ids:
 * 0 - $Click
 */
#[wasm_bindgen]
pub struct WebTicTacToe {
  game: TicTacToeGame,  
  canvas: OffscreenCanvas,
  ctx: CanvasRenderingContext2d, // must be OffscreenCanvasRenderingContext2d, but it's not added in yet
  has_winner: Option<WinningPattern>,
}

#[wasm_bindgen]
impl WebTicTacToe {
  
  #[wasm_bindgen(constructor)]
  pub fn new(canvas: OffscreenCanvas, options: &JsValue) -> Self {

  let ctx = canvas
    .get_context_with_context_options("2d", options)
    .unwrap()
    .unwrap()
    .dyn_into::<CanvasRenderingContext2d>()
    .unwrap();

    Self {
      game: TicTacToeGame::new(PlayerType::Cross, true),
      canvas,
      ctx,
      has_winner: None,
    }
  }

  pub fn update(&mut self) {
    use GameState::*;
    match self.game.state() {
      NoughtWins(pattern) | CrossWins(pattern) => {
        // Dedicate a frame to show that there is a winner, then clear the board on the next update call
        // Note: this only works if we assume that update is called only when needed (ie only when the board state changes)
        if let None = self.has_winner {
          self.has_winner = Some(pattern);
        } else {
          self.has_winner = None;
          self.game = TicTacToeGame::new(PlayerType::Cross, true);
        }
      },
      // If it's a draw just clear the board
      Draw => self.game = TicTacToeGame::new(PlayerType::Cross, true),
      InProgress => ()
    }
  }

  pub fn render(&self) {

    let width = self.canvas.width() as f64; 
    let mut third = width / 3.0;
    let offset = third / 2.0;
    
    // Draw the symbols

    self.ctx.set_font("30px Arial");
    self.ctx.set_text_align("center");
    self.ctx.set_text_baseline("middle");

    let board = self.game.board();
    let mut stride = offset;
    for i in (0..9).step_by(3) {
      self.ctx.fill_text(&board[i].to_string(), offset, stride).expect("Cannot Fill Text");
      self.ctx.fill_text(&board[i + 1].to_string(), offset + third, stride).unwrap();
      self.ctx.fill_text(&board[i + 2].to_string(), offset + third + third, stride).unwrap();
      stride += third;
    }

    self.ctx.begin_path();
    
    // Draw the strikethrough if there is a winner
    if let Some(pattern) = self.has_winner {
      let ((x1, y1), (x2, y2)) = pattern.points();
      self.ctx.move_to(x1 as f64 * third + offset, y1 as f64 * third + offset);
      self.ctx.line_to(x2 as f64 * third + offset, y2 as f64 * third + offset);
    }

    // Draw the grid
    self.ctx.move_to(third, 0.0);
    self.ctx.line_to(third, width);

    self.ctx.move_to(0.0, third);
    self.ctx.line_to(width, third);

    third += third;

    self.ctx.move_to(third, 0.0);
    self.ctx.line_to(third, width);

    self.ctx.move_to(0.0, third);
    self.ctx.line_to(width, third);

    self.ctx.stroke();
  }

  pub fn clear(&self) {
    self.ctx.clear_rect(
      0_f64,
      0_f64,
      self.canvas.width() as f64,
      self.canvas.height() as f64,
    );
  }

  /**
  Note: data must have the shape { x: number, y: number }.
  Typings for plain-object JsValue parameters are not yet supported
  {@link https://github.com/rustwasm/wasm-bindgen/issues/1591}.
  */
  #[wasm_bindgen(js_name = handleInput)]
  pub fn handle_input(&mut self, key: u32, data: &JsValue) {

    let length = self.canvas.width() as f64;
    let third = length / 3.0; 

    match key {
      0 => {
        let data = data.into_serde::<InputData>();
        if let Ok(data) = data {
          let x = data.coordinates.x;
          let y = data.coordinates.y;

          // Normalize the coordinates
          // Note: clamp between 0 and 2 because they are the min and max of row/col indices
          //       floor because we only care about the index
          let x_index = ((x / third).floor() as u32).clamp(0, 2);
          let y_index = ((y / third).floor() as u32).clamp(0, 2);

          // Flatten the index
          let index = y_index * 3 + x_index;

          self.game.do_move(index);

        }
      },
      _ => ()
    }
  }

  // Note: this is made redundant because we are providing the aspect ratio, and control of how it is rendered is given to the renderer
  pub fn resize(&mut self, width: u32, height: u32) {
    // Note: Canvas is a square with side lengths equal to the smallest length of the container
    self.canvas.set_width(width);
    self.canvas.set_height(height);
    self.render();
  }
  
}

#[wasm_bindgen(start)]
pub fn run() {
  utils::set_panic_hook();
}