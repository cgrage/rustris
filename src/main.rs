extern crate pancurses;

use pancurses::{endwin, initscr, noecho, Input, Window};

fn main() {
  // println!("Starting Rustrix");

  let mut board = Board::new(); // model
  let ui = UI::new(); // view
  let game = Game::new(); // controller

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

    game.step(&mut board);
    ui.draw(&board);
  }

  ui.destroy();
}

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
  ActivePiece,
}

#[derive(Copy, Clone)]
enum BlockType {
  I,
  O,
  T,
  J,
  L,
  S,
  Z,
}

impl BlockType {
  fn rand() -> BlockType {
    return BlockType::I;
  }

  fn start_pos(&self) -> (i32, i32) {
    return (0, 0);
  }
}

struct Board {
  cells: [[CellVal; BOARD_DIM_X]; BOARD_DIM_Y],
  block: BlockType,
  block_pos: (i32, i32),
}

impl Board {
  fn new() -> Board {
    let start_block = BlockType::rand();
    return Board {
      cells: [[CellVal::Free; BOARD_DIM_X]; BOARD_DIM_Y],
      block: start_block,
      block_pos: start_block.start_pos(),
    };
  }

  fn width(&self) -> i32 {
    return BOARD_DIM_X as i32;
  }

  fn height(&self) -> i32 {
    return BOARD_DIM_Y as i32;
  }

  fn at(&self, x: i32, y: i32) -> &CellVal {
    if (x, y) == self.block_pos {
      return &CellVal::ActivePiece;
    }
    return &self.cells[y as usize][x as usize];
  }

  fn move_block_down(&mut self) {
    self.block_pos = (self.block_pos.0, self.block_pos.1 + 1);
  }
}

struct UI {
  window: Window,
  board_win: Window,
}

impl UI {
  fn new() -> UI {
    let window = initscr();
    window.refresh();
    window.keypad(false);
    window.timeout(30);
    noecho();
    let board_win = match window.subwin(BOARD_DIM_X as i32, BOARD_DIM_Y as i32, 0, 0) {
      Ok(win) => win,
      Err(code) => panic!("pancurses subwin function failed w/ result code {}", code),
    };
    return UI {
      window: window,
      board_win: board_win,
    };
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
        self.board_win.mv(y, 2 * x);
        self.board_win.printw(self.cell_string(&val));
      }
    }
    self.board_win.touch();
    self.board_win.refresh();
  }

  fn cell_string(&self, val: &CellVal) -> &str {
    return match val {
      CellVal::Free => "  ",
      CellVal::Garbage => "XX",
      CellVal::ActivePiece => "[]",
    };
  }
}

struct Game {}

impl Game {
  fn new() -> Game {
    return Game {};
  }

  fn step(&self, board: &mut Board) {
    board.move_block_down();
  }
}
