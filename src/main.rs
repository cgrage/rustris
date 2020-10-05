extern crate pancurses;

use pancurses::{endwin, initscr, noecho, Input, Window};

const BOARD_DIM_X: usize = 10;
const BOARD_DIM_Y: usize = 20;
const FULL_UI_WIDTH: i32 = 26;
const FULL_UI_HEIGHT: i32 = 26;

enum UserInput {
  UserWantsToQuit,
  NoInput,
}

#[derive(Copy, Clone)]
enum CellVal {
  Free,
  Garbage,
}

struct Board {
  cells: [[CellVal; BOARD_DIM_X]; BOARD_DIM_Y],
}

impl Board {
  fn new() -> Board {
    return Board {
      cells: [[CellVal::Free; BOARD_DIM_X]; BOARD_DIM_Y],
    };
  }

  fn width(&self) -> i32 {
    return BOARD_DIM_X as i32;
  }

  fn height(&self) -> i32 {
    return BOARD_DIM_Y as i32;
  }

  fn at(&self, x: i32, y: i32) -> &CellVal {
    return &self.cells[y as usize][x as usize];
  }
}

struct UI {
  window: Window,
}

impl UI {
  fn new() -> UI {
    return UI { window: initscr() };
  }

  fn init(&self) {
    self.window.refresh();
    self.window.keypad(false);
    self.window.timeout(30);
    noecho();
  }

  fn has_enough_space(&self) -> bool {
    return self.window.get_max_x() >= FULL_UI_WIDTH && self.window.get_max_y() >= FULL_UI_HEIGHT;
  }

  fn space_error(&self) -> String {
    return format!(
      "Not enough space in terminal; need {}x{}, have {}x{}",
      FULL_UI_WIDTH,
      FULL_UI_HEIGHT,
      self.window.get_max_x(),
      self.window.get_max_y(),
    );
  }

  fn destroy(&self) {
    endwin();
  }

  fn wait_for_user_input(&self) -> UserInput {
    let ch = self.window.getch();
    match ch {
      Some(Input::Character('q')) => UserInput::UserWantsToQuit,
      _ => UserInput::NoInput,
    }
  }

  fn draw(&self, board: &Board) {
    for y in 0..board.height() {
      for x in 0..board.width() {
        let val = board.at(x, y);
        self.window.mv(y, 2 * x);
        self.window.printw(self.cell_string(&val));
      }
    }
  }

  fn cell_string(&self, val: &CellVal) -> &str {
    return match val {
      CellVal::Free => "  ",
      CellVal::Garbage => "[]"
    }
  }
}

fn main() {
  // println!("Starting Rustrix");

  let ui = UI::new();
  let board = Board::new();

  ui.init();

  if !ui.has_enough_space() {
    ui.destroy();
    println!("UI init failed: {}", ui.space_error());
    return;
  }

  loop {
    let user_input = ui.wait_for_user_input();
    match user_input {
      UserInput::UserWantsToQuit => break,
      UserInput::NoInput => (),
    }

    ui.draw(&board);
  }

  ui.destroy();
}
