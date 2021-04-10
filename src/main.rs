mod common;
mod game;
mod board;
mod curses_ui;

use common::{UiState, UserInput};
use board::Board;
use game::Game;
use curses_ui::UI;

fn main() {
  let ui_result = UI::new();
  if ui_result.is_err() {
    println!("UI init failed: {}", ui_result.unwrap_err());
    return;
  }

  // tests done, let's go
  let mut board = Board::new(); // models
  let mut ui_state = UiState::new();
  let ui = ui_result.unwrap(); // view
  let game = Game::new(); // controller

  loop {
    let user_input = ui.wait_for_user_input();
    match user_input {
      UserInput::UserWantsToQuit => break,
      UserInput::ChangeUI => ui.change(&mut ui_state),
      input => game.handle_input(&input, &mut board),
    }

    game.step(&mut board);
    ui.draw(&board);
  }

  ui.destroy();
}
