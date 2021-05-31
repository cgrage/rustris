mod curses_ui;

use crate::curses_ui::{UiInput, UiState, UI};
use rustris_core::game::Game;
use std::time::{Duration, Instant};

const SLEEP_TIME: Duration = Duration::from_millis(0);
const FRAME_TIME: Duration = Duration::from_nanos(16666667);

fn main() {
  let ui_result = UI::new();
  if ui_result.is_err() {
    println!("UI init failed: {}", ui_result.unwrap_err());
    return;
  }

  let mut ui_state = UiState::new();
  let mut ui = ui_result.unwrap();
  let mut game = Game::new();

  loop {
    let t_start = Instant::now();
    let (user_input, ui_input) = ui.read_user_input();

    match ui_input {
      UiInput::UserWantsToQuit => break,
      UiInput::ChangeUI => ui.change(&mut ui_state),
      _ => (),
    }

    game.handle_input(&user_input);
    game.run_step();
    ui.draw(&game);

    while Instant::now() - t_start < FRAME_TIME {
      std::thread::sleep(SLEEP_TIME);
    }
  }

  ui.destroy();
}
