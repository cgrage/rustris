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

enum BlockType {
  I,
  O,
  T,
  J,
  L,
  S,
  Z,
}

enum BlockRot {
  Rot0,
  Rot1,
  Rot2,
  Rot3,
}

struct Board {
  cells: [[CellVal; BOARD_DIM_X]; BOARD_DIM_Y],
  block: Block,
  time: u32,
  time_to_drop: u32,
  drop_interval: u32,
}

impl Board {
  fn new() -> Board {
    return Board {
      cells: [[CellVal::Free; BOARD_DIM_X]; BOARD_DIM_Y],
      block: Block::rand(),
      time: 0,
      time_to_drop: 10,
      drop_interval: 10,
    };
  }

  fn width(&self) -> i32 {
    return BOARD_DIM_X as i32;
  }

  fn height(&self) -> i32 {
    return BOARD_DIM_Y as i32;
  }

  fn at(&self, x: i32, y: i32) -> &CellVal {
    if self.block.probe(x, y) {
      return &CellVal::ActivePiece;
    }
    return &self.cells[y as usize][x as usize];
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
    if board.time_to_drop == 0 {
      board.block.move_down();
      board.time_to_drop = board.drop_interval;
    } else {
      board.time_to_drop = board.time_to_drop - 1;
    }
    board.time = board.time + 1;
  }
}

// -----------------------------------------------------------------------------

struct Block {
  b_type: BlockType,
  b_rot: BlockRot,
  b_pos: (i32, i32),
}

impl Block {
  fn rand() -> Block {
    return Block {
      b_type: BlockType::I,
      b_rot: BlockRot::Rot0,
      b_pos: (BOARD_DIM_X as i32 / 2, 0),
    };
  }

  fn move_down(&mut self) {
    self.b_pos = (self.b_pos.0, self.b_pos.1 + 1);
  }

  fn probe(&self, board_x: i32, board_y: i32) -> bool {
    if self.b_pos.0 - board_x < 0 || self.b_pos.1 - board_y < 0 {
      return false;
    }

    let x = (self.b_pos.0 - board_x) as usize;
    let y = (self.b_pos.1 - board_y) as usize;

    return match self.b_type {
      BlockType::I | BlockType::O => {
        x < 4 && y < 4 && Block::pattern_4x4(&self.b_type, &self.b_rot)[y][x] == 1
      }
      BlockType::T | BlockType::J | BlockType::L | BlockType::S | BlockType::Z => {
        x < 3 && y < 3 && Block::pattern_3x3(&self.b_type, &self.b_rot)[y][x] == 1
      }
    };
  }

  fn pattern_4x4(b_type: &BlockType, b_rot: &BlockRot) -> [[i32; 4]; 4] {
    return match b_type {
      BlockType::I => match b_rot {
        BlockRot::Rot0 => [
          [0, 0, 0, 0], //
          [1, 1, 1, 1],
          [0, 0, 0, 0],
          [0, 0, 0, 0],
        ],
        BlockRot::Rot1 => [
          [0, 0, 1, 0], //
          [0, 0, 1, 0],
          [0, 0, 1, 0],
          [0, 0, 1, 0],
        ],
        BlockRot::Rot2 => [
          [0, 0, 0, 0], //
          [0, 0, 0, 0],
          [1, 1, 1, 1],
          [0, 0, 0, 0],
        ],
        BlockRot::Rot3 => [
          [0, 1, 0, 0], //
          [0, 1, 0, 0],
          [0, 1, 0, 0],
          [0, 1, 0, 0],
        ],
      },
      BlockType::O => [
        [0, 0, 0, 0], //
        [0, 1, 1, 0],
        [0, 1, 1, 0],
        [0, 0, 0, 0],
      ],
      _ => panic!("unknown 4x4 type"),
    };
  }

  fn pattern_3x3(b_type: &BlockType, b_rot: &BlockRot) -> [[i32; 3]; 3] {
    return match b_type {
      BlockType::T => match b_rot {
        BlockRot::Rot0 => [
          [0, 1, 0], //
          [1, 1, 1],
          [0, 0, 0],
        ],
        BlockRot::Rot1 => [
          [0, 1, 0], //
          [0, 1, 1],
          [0, 1, 0],
        ],
        BlockRot::Rot2 => [
          [0, 0, 0], //
          [1, 1, 1],
          [0, 1, 0],
        ],
        BlockRot::Rot3 => [
          [0, 1, 0], //
          [1, 1, 0],
          [0, 1, 0],
        ],
      },
      BlockType::J => [
        [0, 0, 0], //
        [0, 0, 0],
        [0, 0, 0],
      ],
      BlockType::L => [
        [0, 0, 0], //
        [0, 0, 0],
        [0, 0, 0],
      ],
      BlockType::S => [
        [0, 0, 0], //
        [0, 0, 0],
        [0, 0, 0],
      ],
      BlockType::Z => [
        [0, 0, 0], //
        [0, 0, 0],
        [0, 0, 0],
      ],
      _ => panic!("unknown 3x3 type"),
    };
  }
}
