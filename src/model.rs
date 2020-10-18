extern crate rand;

use self::rand::Rng;

const BOARD_DIM_X: usize = 10;
const BOARD_DIM_Y: usize = 20;

#[derive(Copy, Clone)]
pub enum CellVal {
  Free,
  Garbage,
  ActivePiece,
  OutOfBoard,
}

#[derive(Copy, Clone)]
pub enum BlockType {
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

pub struct Board {
  cells: [[CellVal; BOARD_DIM_X]; BOARD_DIM_Y],
  block_type: BlockType,
  block_rot: BlockRot,
  block_pos: (i32, i32),
  pub time: u32,
  pub time_to_step: u32,
  pub step_interval: u32,
  pub next_block_type: BlockType,
}

impl Board {
  pub fn new() -> Board {
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

  pub fn width(&self) -> i32 {
    return BOARD_DIM_X as i32;
  }

  pub fn height(&self) -> i32 {
    return BOARD_DIM_Y as i32;
  }

  pub fn cell_value(&self, x: i32, y: i32) -> &CellVal {
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

  pub fn move_block(&mut self, x: i32, y: i32) {
    self.block_pos = (self.block_pos.0 + x, self.block_pos.1 + y);
  }

  pub fn rotate_block(&mut self, r: i32) {
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

  pub fn collides(&self) -> bool {
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

  pub fn freeze_block(&mut self) {
    // only check a 4x4 area at the blocks pos
    for y in self.block_pos.1..self.block_pos.1 + 4 {
      for x in self.block_pos.0..self.block_pos.0 + 4 {
        if self.probe_block_at(x, y) {
          self.add_garbage(x, y);
        }
      }
    }
  }

  pub fn next_block(&mut self) {
    self.block_type = self.next_block_type;
    self.block_rot = BlockRot::Rot0;
    self.block_pos = (BOARD_DIM_X as i32 / 2, 0);
    self.next_block_type = BlockType::rand();
  }

  fn is_row_full(&self, row: i32) -> bool {
    for x in 0..self.width() {
      match self.cell_value_raw(x, row) {
        CellVal::Free => return false,
        _ => (),
      }
    }
    return true;
  }

  fn remove_row(&mut self, row: i32) {
    for y in (0..row).rev() {
      for x in 0..self.width() {
        self.cells[(y + 1) as usize][x as usize] = self.cells[y as usize][x as usize];
      }
    }
    for x in 0..self.width() {
      self.cells[0][x as usize] = CellVal::Free;
    }
  }

  pub fn clear_full_rows(&mut self) -> i32 {
    let mut count = 0;
    for y in 0..self.height() {
      if self.is_row_full(y) {
        self.remove_row(y);
        count = count + 1;
      }
    }
    return count;
  }
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
        [1, 0, 0], //
        [1, 1, 1],
        [0, 0, 0],
      ],
      BlockRot::Rot1 => [
        [0, 1, 1], //
        [0, 1, 0],
        [0, 1, 0],
      ],
      BlockRot::Rot2 => [
        [0, 0, 0], //
        [1, 1, 1],
        [0, 0, 1],
      ],
      BlockRot::Rot3 => [
        [0, 1, 0], //
        [0, 1, 0],
        [1, 1, 0],
      ],
    },
    BlockType::L => match b_rot {
      BlockRot::Rot0 => [
        [0, 0, 1], //
        [1, 1, 1],
        [0, 0, 0],
      ],
      BlockRot::Rot1 => [
        [0, 1, 0], //
        [0, 1, 0],
        [0, 1, 1],
      ],
      BlockRot::Rot2 => [
        [0, 0, 0], //
        [1, 1, 1],
        [1, 0, 0],
      ],
      BlockRot::Rot3 => [
        [1, 1, 0], //
        [0, 1, 0],
        [0, 1, 0],
      ],
    },
    BlockType::S => match b_rot {
      BlockRot::Rot0 => [
        [0, 1, 1], //
        [1, 1, 0],
        [0, 0, 0],
      ],
      BlockRot::Rot1 => [
        [0, 1, 0], //
        [0, 1, 1],
        [0, 0, 1],
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
        [1, 1, 0], //
        [0, 1, 1],
        [0, 0, 0],
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
        [0, 1, 0], //
        [1, 1, 0],
        [1, 0, 0],
      ],
    },
    _ => panic!("unknown 3x3 type"),
  };
}
