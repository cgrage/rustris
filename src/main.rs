extern crate pancurses;

use pancurses::{endwin, initscr, noecho, Input, Window};

const PLAYFIELD_DIM_X: i32 = 10;
const PLAYFIELD_DIM_Y: i32 = 20;
const FULL_UI_DIM_X: i32 = 26;
const FULL_UI_DIM_Y: i32 = 26;

enum UserInput {
  UserWantsToQuit,
  NoInput,
}

struct UI {
  window: Window,
}

fn main() {
  let ui = match init_ui() {
    Ok(win) => win,
    Err(msg) => panic!("UI init failed: {}", msg),
  };

  loop {
    let user_input = wait_for_user_input(&ui);

    match user_input {
      UserInput::UserWantsToQuit => break,
      UserInput::NoInput => (),
    }

    draw_ui(&ui);
  }

  destroy_ui(&ui);
}

fn init_ui() -> Result<UI, String> {
  let window = initscr();
  if window.get_max_x() < FULL_UI_DIM_X || window.get_max_y() < FULL_UI_DIM_Y {
    endwin();
    return Err(format!(
      "Not enough space in terminal; need {}x{}, have {}x{}",
      FULL_UI_DIM_X,
      FULL_UI_DIM_Y,
      window.get_max_x(),
      window.get_max_y(),
    ));
  }

  window.refresh();
  window.keypad(true);
  window.timeout(30);
  noecho();
  return Ok(UI { window: window });
}

fn destroy_ui(_ui: &UI) {
  endwin();
}

fn wait_for_user_input(ui: &UI) -> UserInput {
  let ch = ui.window.getch();
  match ch {
    Some(Input::Character('q')) => UserInput::UserWantsToQuit,
    _ => UserInput::NoInput,
  }
}

fn draw_ui(_ui: &UI) {
  //
}
