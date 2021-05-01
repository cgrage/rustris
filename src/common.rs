extern crate rand;

use self::rand::Rng;

#[derive(Copy, Clone)]
pub enum CellVal {
  Free,
  OutOfBoard,
  Color0,
  Color1,
  Color2,
  Color3,
  Color4,
  Color5,
  Color6,
  Color7,
  Color8,
  Color9,
}

impl CellVal {
  pub fn rand_color() -> CellVal {
    let mut rnd = rand::thread_rng();
    return match rnd.gen_range(0, 10) {
      0 => CellVal::Color0,
      1 => CellVal::Color1,
      2 => CellVal::Color2,
      3 => CellVal::Color3,
      4 => CellVal::Color4,
      5 => CellVal::Color5,
      6 => CellVal::Color6,
      7 => CellVal::Color7,
      8 => CellVal::Color8,
      _ => CellVal::Color9,
    };
  }
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
    return UiState { style: 0 };
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
  Reset,
  NoInput,
}

pub struct Stats {
  pub cleared: i32,
  pub clr_cmb_4: i32,
  pub clr_cmb_3: i32,
  pub clr_cmb_2: i32,
  pub clr_cmb_1: i32,
}

impl Stats {
  pub fn new() -> Stats {
    return Stats {
      cleared: 0,
      clr_cmb_4: 0,
      clr_cmb_3: 0,
      clr_cmb_2: 0,
      clr_cmb_1: 0,
    };
  }

  pub fn reset(&mut self) {
    self.cleared = 0;
    self.clr_cmb_4 = 0;
    self.clr_cmb_3 = 0;
    self.clr_cmb_2 = 0;
    self.clr_cmb_1 = 0;
  }
}
