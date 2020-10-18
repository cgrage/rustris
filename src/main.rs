extern crate pancurses;
extern crate rand;

use rand::Rng;

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
      input => game.handle_input(&input, &mut board),
    }

    game.step(&mut board);
    ui.draw_background(&board);
    ui.draw_foreground(&board);
  }

  ui.destroy();
}

const BOARD_DIM_X: usize = 10;
const BOARD_DIM_Y: usize = 20;

enum UserInput {
  UserWantsToQuit,
  MoveLeft,
  MoveRight,
  MoveDown,
  DropDown,
  RotateLeft,
  RotateRight,
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
  time_to_step: u32,
  step_interval: u32,
}

impl Board {
  fn new() -> Board {
    return Board {
      cells: [[CellVal::Free; BOARD_DIM_X]; BOARD_DIM_Y],
      block: Block::rand(),
      time: 0,
      time_to_step: 10,
      step_interval: 10,
    };
  }

  fn width(&self) -> i32 {
    return BOARD_DIM_X as i32;
  }

  fn height(&self) -> i32 {
    return BOARD_DIM_Y as i32;
  }

  fn at(&self, x: i32, y: i32) -> &CellVal {
    if self.block.probe_is_filled(x, y) {
      return &CellVal::ActivePiece;
    }
    return &self.cells[y as usize][x as usize];
  }

  fn move_block_vert(&mut self, amount : i32) {
    self.block.move_block(amount, 0)
  }

  fn lower_block(&mut self) {
    self.block.move_block(0, 1)
  }

  fn drop_block(&mut self) {
    //
  }

  fn rotate_block(&mut self, amount : i32) {
    self.block.rotate_block(amount);
  }
}

struct UI {
  global_win: pancurses::Window,
  game_win: pancurses::Window,
  board_win: pancurses::Window,
}

impl UI {
  const BOARD_BORDER: i32 = 1;
  const FULL_UI_WIDTH: i32 = 26;
  const FULL_UI_HEIGHT: i32 = 26;

  fn new() -> UI {
    let global_win = pancurses::initscr();
    global_win.clear();
    global_win.timeout(30);
    global_win.keypad(true);
    pancurses::noecho();
    pancurses::cbreak();
    pancurses::curs_set(0);

    let game_win = match global_win.subwin(UI::FULL_UI_HEIGHT, UI::FULL_UI_WIDTH, 0, 0) {
      Ok(win) => win,
      Err(code) => panic!("pancurses subwin function failed w/ result code {}", code),
    };

    let board_with = (BOARD_DIM_X * 2) as i32 + 2 * UI::BOARD_BORDER;
    let board_height = BOARD_DIM_Y as i32 + 2 * UI::BOARD_BORDER;
    let board_win = match game_win.subwin(board_height, board_with, 0, 0) {
      Ok(win) => win,
      Err(code) => panic!("pancurses subwin function failed w/ result code {}", code),
    };
    board_win.border('|', '|', '-', '-', '+', '+', '+', '+');

    return UI {
      global_win: global_win,
      game_win: game_win,
      board_win: board_win,
    };
  }

  fn has_enough_space(&self) -> bool {
    return self.global_win.get_max_x() >= UI::FULL_UI_WIDTH
      && self.global_win.get_max_y() >= UI::FULL_UI_HEIGHT;
  }

  fn space_error(&self) -> String {
    return format!(
      "Not enough space in terminal; need {}x{}, have {}x{}",
      UI::FULL_UI_WIDTH,
      UI::FULL_UI_HEIGHT,
      self.global_win.get_max_x(),
      self.global_win.get_max_y(),
    );
  }

  fn destroy(&self) {
    pancurses::endwin();
  }

  fn wait_for_user_input(&self) -> UserInput {
    let ch = self.global_win.getch();
    match ch {
      Some(pancurses::Input::Character('q')) => UserInput::UserWantsToQuit,
      Some(pancurses::Input::Unknown(27)) => UserInput::UserWantsToQuit, // 27 is ESC key
      Some(pancurses::Input::KeyLeft) => UserInput::RotateLeft,
      Some(pancurses::Input::KeyRight) => UserInput::RotateRight,
      Some(pancurses::Input::Character('a')) => UserInput::MoveLeft,
      Some(pancurses::Input::Character('d')) => UserInput::MoveRight,
      Some(pancurses::Input::Character('s')) => UserInput::MoveDown,
      Some(pancurses::Input::Character('w')) => UserInput::DropDown,
      _ => UserInput::NoInput,
    }
  }

  fn draw_background(&self, _board: &Board) {
    self.game_win.touch();
    //self.game_win.refresh();
  }

  fn draw_foreground(&self, board: &Board) {
    let b = UI::BOARD_BORDER;
    for y in 0..board.height() {
      for x in 0..board.width() {
        let val = board.at(x, y);
        self.board_win.mv(b + y, b + 2 * x);
        self.board_win.printw(self.cell_string(&val));
      }
    }
    self.board_win.touch();
    self.game_win.refresh();
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
    if board.time_to_step == 0 {
      board.block.move_block(0, 1);
      board.time_to_step = board.step_interval;
    } else {
      board.time_to_step = board.time_to_step - 1;
    }
    board.time = board.time + 1;
  }

  fn handle_input(&self, input: &UserInput, board: &mut Board) {
    match input {
      UserInput::MoveLeft => board.move_block_vert(-1),
      UserInput::MoveRight => board.move_block_vert(1),
      UserInput::MoveDown => board.lower_block(),
      UserInput::DropDown => board.drop_block(),
      UserInput::RotateLeft => board.rotate_block(-1),
      UserInput::RotateRight => board.rotate_block(1),
      _ => (),
    };
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
      b_type: Block::rand_block_type(),
      b_rot: BlockRot::Rot0,
      b_pos: (BOARD_DIM_X as i32 / 2, 0),
    };
  }

  fn rand_block_type() -> BlockType {
    let mut rnd = rand::thread_rng();
    return match rnd.gen_range(0, 7) {
      0 => BlockType::I,
      1 => BlockType::O,
      2 => BlockType::T,
      3 => BlockType::J,
      4 => BlockType::L,
      5 => BlockType::S,
      _ => BlockType::Z,
    };
  }

  fn move_block(&mut self, x: i32, y: i32) {
    self.b_pos = (self.b_pos.0 + x, self.b_pos.1 + y);
  }

  fn rotate_block(&mut self, r: i32) {
    match r {
      1 => {
        self.b_rot = match self.b_rot {
          BlockRot::Rot0 => BlockRot::Rot1,
          BlockRot::Rot1 => BlockRot::Rot2,
          BlockRot::Rot2 => BlockRot::Rot3,
          BlockRot::Rot3 => BlockRot::Rot0,
        }
      }
      -1 => {
        self.b_rot = match self.b_rot {
          BlockRot::Rot0 => BlockRot::Rot3,
          BlockRot::Rot1 => BlockRot::Rot0,
          BlockRot::Rot2 => BlockRot::Rot1,
          BlockRot::Rot3 => BlockRot::Rot2,
        }
      }
      _ => panic!("rotate_block must be called w/ 1 or -1"),
    }
  }

  fn probe_is_filled(&self, board_x: i32, board_y: i32) -> bool {
    if board_x - self.b_pos.0 < 0 || board_y - self.b_pos.1 < 0 {
      return false;
    }

    let dx = (board_x - self.b_pos.0) as usize;
    let dy = (board_y - self.b_pos.1) as usize;

    return match self.b_type {
      BlockType::I | BlockType::O => {
        dx < 4 && dy < 4 && Block::pattern_4x4(&self.b_type, &self.b_rot)[dy][dx] == 1
      }
      BlockType::T | BlockType::J | BlockType::L | BlockType::S | BlockType::Z => {
        dx < 3 && dy < 3 && Block::pattern_3x3(&self.b_type, &self.b_rot)[dy][dx] == 1
      }
    };
  }

  // from: https://strategywiki.org/wiki/Tetris/Rotation_systems
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
      BlockType::J => match b_rot {
        BlockRot::Rot0 => [
          [0, 0, 0], //
          [1, 1, 1],
          [0, 0, 1],
        ],
        BlockRot::Rot1 => [
          [0, 1, 0], //
          [0, 1, 0],
          [1, 1, 0],
        ],
        BlockRot::Rot2 => [
          [0, 0, 0], //
          [1, 0, 0],
          [1, 1, 1],
        ],
        BlockRot::Rot3 => [
          [0, 1, 1], //
          [0, 1, 0],
          [0, 1, 0],
        ],
      },
      BlockType::L => match b_rot {
        BlockRot::Rot0 => [
          [0, 0, 0], //
          [1, 1, 1],
          [1, 0, 0],
        ],
        BlockRot::Rot1 => [
          [1, 1, 0], //
          [0, 1, 0],
          [0, 1, 0],
        ],
        BlockRot::Rot2 => [
          [0, 0, 0], //
          [0, 0, 1],
          [1, 1, 1],
        ],
        BlockRot::Rot3 => [
          [0, 1, 0], //
          [0, 1, 0],
          [0, 1, 1],
        ],
      },
      BlockType::S => match b_rot {
        BlockRot::Rot0 => [
          [0, 0, 0], //
          [0, 1, 1],
          [1, 1, 0],
        ],
        BlockRot::Rot1 => [
          [1, 0, 0], //
          [1, 1, 0],
          [0, 1, 0],
        ],
        BlockRot::Rot2 => [
          [0, 0, 0], //
          [0, 1, 1],
          [1, 1, 0],
        ],
        BlockRot::Rot3 => [
          [1, 0, 0], //
          [1, 1, 0],
          [0, 1, 0],
        ],
      },
      BlockType::Z => match b_rot {
        BlockRot::Rot0 => [
          [0, 0, 0], //
          [1, 1, 0],
          [0, 1, 1],
        ],
        BlockRot::Rot1 => [
          [0, 0, 1], //
          [0, 1, 1],
          [0, 1, 0],
        ],
        BlockRot::Rot2 => [
          [0, 0, 0], //
          [1, 1, 0],
          [0, 1, 1],
        ],
        BlockRot::Rot3 => [
          [0, 0, 1], //
          [0, 1, 1],
          [0, 1, 0],
        ],
      },
      _ => panic!("unknown 3x3 type"),
    };
  }
}
