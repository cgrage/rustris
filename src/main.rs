extern crate pancurses;
extern crate rand;

use rand::Rng;

fn main() {
  // println!("Starting Rustrix");

  let ui_result = UI::new();
  if ui_result.is_err() {
    println!("UI init failed: {}", ui_result.unwrap_err());
    return;
  }

  // checks done. let's go
  let mut board = Board::new(); // model
  let ui = ui_result.unwrap(); // view
  let game = Game::new(); // controller

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
  OutOfBoard,
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
}

enum BlockRot {
  Rot0,
  Rot1,
  Rot2,
  Rot3,
}

struct Board {
  cells: [[CellVal; BOARD_DIM_X]; BOARD_DIM_Y],
  block_type: BlockType,
  block_rot: BlockRot,
  block_pos: (i32, i32),
  time: u32,
  time_to_step: u32,
  step_interval: u32,
  next_block_type: BlockType,
}

impl Board {
  fn new() -> Board {
    return Board {
      cells: [[CellVal::Free; BOARD_DIM_X]; BOARD_DIM_Y],
      block_type: BlockType::rand(),
      block_rot: BlockRot::Rot0,
      block_pos: (BOARD_DIM_X as i32 / 2, 0),
      time: 0,
      time_to_step: 10,
      step_interval: 10,
      next_block_type: BlockType::rand(),
    };
  }

  fn width(&self) -> i32 {
    return BOARD_DIM_X as i32;
  }

  fn height(&self) -> i32 {
    return BOARD_DIM_Y as i32;
  }

  fn cell_value(&self, x: i32, y: i32) -> &CellVal {
    if self.probe_block_at(x, y) {
      return &CellVal::ActivePiece;
    }
    return self.cell_value_raw(x, y);
  }

  fn cell_value_raw(&self, x: i32, y: i32) -> &CellVal {
    if x < 0 || x >= BOARD_DIM_X as i32 || y < 0 || y >= BOARD_DIM_Y as i32 {
      return &CellVal::OutOfBoard;
    }
    return &self.cells[y as usize][x as usize];
  }

  fn move_block(&mut self, x: i32, y: i32) {
    self.block_pos = (self.block_pos.0 + x, self.block_pos.1 + y);
  }

  fn rotate_block(&mut self, r: i32) {
    match r {
      1 => {
        self.block_rot = match self.block_rot {
          BlockRot::Rot0 => BlockRot::Rot1,
          BlockRot::Rot1 => BlockRot::Rot2,
          BlockRot::Rot2 => BlockRot::Rot3,
          BlockRot::Rot3 => BlockRot::Rot0,
        }
      }
      -1 => {
        self.block_rot = match self.block_rot {
          BlockRot::Rot0 => BlockRot::Rot3,
          BlockRot::Rot1 => BlockRot::Rot0,
          BlockRot::Rot2 => BlockRot::Rot1,
          BlockRot::Rot3 => BlockRot::Rot2,
        }
      }
      _ => panic!("rotate_block must be called w/ 1 or -1"),
    }
  }

  fn add_garbage(&mut self, x: i32, y: i32) {
    self.cells[y as usize][x as usize] = CellVal::Garbage;
  }

  fn probe_block_at(&self, board_x: i32, board_y: i32) -> bool {
    if board_x - self.block_pos.0 < 0 || board_y - self.block_pos.1 < 0 {
      return false;
    }

    let dx = (board_x - self.block_pos.0) as usize;
    let dy = (board_y - self.block_pos.1) as usize;

    return match self.block_type {
      BlockType::I | BlockType::O => {
        dx < 4 && dy < 4 && pattern_4x4(&self.block_type, &self.block_rot)[dy][dx] == 1
      }
      BlockType::T | BlockType::J | BlockType::L | BlockType::S | BlockType::Z => {
        dx < 3 && dy < 3 && pattern_3x3(&self.block_type, &self.block_rot)[dy][dx] == 1
      }
    };
  }

  fn collides(&self) -> bool {
    // only check a 4x4 area at the blocks pos
    for y in self.block_pos.1..self.block_pos.1 + 4 {
      for x in self.block_pos.0..self.block_pos.0 + 4 {
        if self.probe_block_at(x, y) {
          match self.cell_value_raw(x, y) {
            CellVal::Free => (/* free */),
            _ => return true,
          }
        }
      }
    }
    return false;
  }

  fn freeze_block(&mut self) {
    // only check a 4x4 area at the blocks pos
    for y in self.block_pos.1..self.block_pos.1 + 4 {
      for x in self.block_pos.0..self.block_pos.0 + 4 {
        if self.probe_block_at(x, y) {
          self.add_garbage(x, y);
        }
      }
    }
  }

  fn next_block(&mut self) {
    self.block_type = self.next_block_type;
    self.block_rot = BlockRot::Rot0;
    self.block_pos = (BOARD_DIM_X as i32 / 2, 0);
    self.next_block_type = BlockType::rand();
  }

  fn clear_full_rows(&self) -> i32 {
    return 0;
  }
}

#[derive(Debug)]
struct UI {
  screen: pancurses::Window,
  game_win: pancurses::Window,
  board_win: pancurses::Window,
}

impl UI {
  const BOARD_BORDER: i32 = 1;
  const FULL_UI_WIDTH: i32 = 26;
  const FULL_UI_HEIGHT: i32 = 26;

  fn new() -> Result<UI, String> {
    let screen = pancurses::initscr();

    if screen.get_max_x() < UI::FULL_UI_WIDTH || screen.get_max_y() < UI::FULL_UI_HEIGHT {
      pancurses::endwin();
      return Err(format!(
        "Not enough space in terminal; need {}x{}, have {}x{}",
        UI::FULL_UI_WIDTH,
        UI::FULL_UI_HEIGHT,
        screen.get_max_x(),
        screen.get_max_y()
      ));
    }

    screen.clear();
    screen.timeout(30);
    screen.keypad(true);
    pancurses::noecho();
    pancurses::cbreak();
    pancurses::curs_set(0);

    let game_win_result = screen.subwin(UI::FULL_UI_HEIGHT, UI::FULL_UI_WIDTH, 0, 0);
    if game_win_result.is_err() {
      pancurses::endwin();
      return Err(format!(
        "pancurses subwin function failed w/ result code {}",
        game_win_result.unwrap_err()
      ));
    }

    let game_win = game_win_result.unwrap();
    let board_with = (BOARD_DIM_X * 2) as i32 + 2 * UI::BOARD_BORDER;
    let board_height = BOARD_DIM_Y as i32 + 2 * UI::BOARD_BORDER;

    let board_win_result = game_win.subwin(board_height, board_with, 0, 0);
    if board_win_result.is_err() {
      pancurses::endwin();
      return Err(format!(
        "pancurses subwin function failed w/ result code {}",
        board_win_result.unwrap_err()
      ));
    };

    let board_win = board_win_result.unwrap();
    board_win.border('|', '|', '-', '-', '+', '+', '+', '+');

    return Ok(UI {
      screen: screen,
      game_win: game_win,
      board_win: board_win,
    });
  }

  fn destroy(&self) {
    pancurses::endwin();
  }

  fn wait_for_user_input(&self) -> UserInput {
    let ch = self.screen.getch();
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
        let val = board.cell_value(x, y);
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
      CellVal::Garbage => "[]",
      CellVal::ActivePiece => "[]",
      CellVal::OutOfBoard => "??",
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
      self.lower_block(board);
      board.time_to_step = board.step_interval;
    } else {
      board.time_to_step = board.time_to_step - 1;
    }
    board.time = board.time + 1;
  }

  fn handle_input(&self, input: &UserInput, board: &mut Board) {
    match input {
      UserInput::MoveLeft => self.move_block_vert(board, -1),
      UserInput::MoveRight => self.move_block_vert(board, 1),
      UserInput::MoveDown => self.lower_block(board),
      UserInput::DropDown => self.drop_block(board),
      UserInput::RotateLeft => self.rotate_block(board, -1),
      UserInput::RotateRight => self.rotate_block(board, 1),
      _ => (),
    };
  }

  fn move_block_vert(&self, board: &mut Board, amount: i32) {
    board.move_block(amount, 0);
    if board.collides() {
      // if we collide: undo action
      board.move_block(-amount, 0);
    }
  }

  fn rotate_block(&self, board: &mut Board, amount: i32) {
    board.rotate_block(amount);
    if board.collides() {
      // if we collide: undo action
      board.rotate_block(-amount);
    }
  }

  fn lower_block(&self, board: &mut Board) {
    board.move_block(0, 1);
    if board.collides() {
      // if we collide: undo action and FREEZE block
      board.move_block(0, -1);
      self.freeze_block_and_have_next(board);
    }
  }

  fn drop_block(&self, board: &mut Board) {
    while !board.collides() {
      board.move_block(0, 1);
    }
    board.move_block(0, -1);
    self.freeze_block_and_have_next(board);
  }

  fn freeze_block_and_have_next(&self, board: &mut Board) {
    board.freeze_block();
    /*let row_count =*/
    board.clear_full_rows();
    board.next_block();
  }
}

// -----------------------------------------------------------------------------

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
