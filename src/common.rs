extern crate rand;

use self::rand::Rng;

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
  pub fn rand() -> BlockType {
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

pub enum BlockRot {
  Rot0,
  Rot1,
  Rot2,
  Rot3,
}


pub struct UiState {
  pub style: i32,
}

impl UiState {
  pub fn new() -> UiState {
    return UiState { 
      style: 0
    }
  }
}

pub enum UserInput {
  UserWantsToQuit,
  MoveLeft,
  MoveRight,
  MoveDown,
  DropDown,
  RotateLeft,
  RotateRight,
  ChangeUI,
  NoInput,
}
