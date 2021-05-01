mod board;
mod common;
mod curses_ui;
mod game;

use crate::board::Board;
use crate::common::{Stats, UiState, UserInput};
use crate::curses_ui::UI;
use crate::game::Game;
use std::time::{Duration, Instant};

const SLEEP_TIME: Duration = Duration::from_millis(0);
const FRAME_TIME: Duration = Duration::from_nanos(16666667);

fn main() {
  let ui_result = UI::new();
  if ui_result.is_err() {
    println!("UI init failed: {}", ui_result.unwrap_err());
    return;
  }

  // tests done, let's go
  let mut board = Board::new(); // models
  let mut ui_state = UiState::new();
  let mut stats = Stats::new();
  let mut ui = ui_result.unwrap(); // view
  let mut game = Game::new(); // controller

  loop {
    let t_start = Instant::now();
    let user_input = ui.read_user_input();

    match user_input {
      UserInput::UserWantsToQuit => break,
      UserInput::ChangeUI => ui.change(&mut ui_state),
      input => game.handle_input(&input, &mut board, &mut stats),
    }

    game.step(&mut board, &mut stats);
    ui.draw(&board, &stats);

    while Instant::now() - t_start < FRAME_TIME {
      std::thread::sleep(SLEEP_TIME);
    }
  }

  ui.destroy();
}
