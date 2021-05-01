extern crate rand;

use self::rand::Rng;
use common::CellVal;

const BOARD_DIM_X: usize = 10;
const BOARD_DIM_Y: usize = 20;

pub struct Board {
    cells: [[CellVal; BOARD_DIM_X]; BOARD_DIM_Y],
    block_type: BlockType,
    block_color: CellVal,
    block_rot: BlockRot,
    block_offset: (i32, i32),
    next_block_type: BlockType,
    next_block_color: CellVal,
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

enum BlockRot {
    Rot0,
    Rot1,
    Rot2,
    Rot3,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            cells: [[CellVal::Free; BOARD_DIM_X]; BOARD_DIM_Y],
            block_type: BlockType::I,
            block_color: CellVal::Color0,
            block_rot: BlockRot::Rot0,
            block_offset: (0, 0),
            next_block_type: BlockType::rand(),
            next_block_color: CellVal::rand_color(),
        };
        board.next_block();
        return board;
    }

    pub fn clear(&mut self) {
        self.cells = [[CellVal::Free; BOARD_DIM_X]; BOARD_DIM_Y];
    }

    pub fn width(&self) -> i32 {
        return BOARD_DIM_X as i32;
    }

    pub fn height(&self) -> i32 {
        return BOARD_DIM_Y as i32;
    }

    pub fn cell_value_at_board(&self, x: i32, y: i32) -> &CellVal {
        if x < 0 || x >= BOARD_DIM_X as i32 || y < 0 || y >= BOARD_DIM_Y as i32 {
            return &CellVal::OutOfBoard;
        } else {
            return &self.cells[y as usize][x as usize];
        }
    }

    pub fn cell_value_at_active_block(&self, x: i32, y: i32) -> &CellVal {
        if self.probe_active_block_at(x, y) {
            return &self.block_color;
        } else {
            return &CellVal::Free;
        }
    }

    pub fn active_block_pos_to_board_pos(&self, x: i32, y: i32) -> (i32, i32) {
        return (&self.block_offset.0 + x, &self.block_offset.1 + y);
    }

    pub fn cell_value_at_next_block(&self, x: i32, y: i32) -> &CellVal {
        if self.probe_next_block_at(x, y) {
            return &self.next_block_color;
        } else {
            return &CellVal::Free;
        }
    }

    pub fn move_block(&mut self, x: i32, y: i32) {
        self.block_offset = (self.block_offset.0 + x, self.block_offset.1 + y);
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

    fn set_call_value(&mut self, x: i32, y: i32, value: CellVal) {
        self.cells[y as usize][x as usize] = value;
    }

    fn probe_active_block_at(&self, x: i32, y: i32) -> bool {
        return probe(&self.block_type, &self.block_rot, x, y);
    }

    fn probe_next_block_at(&self, x: i32, y: i32) -> bool {
        return probe(&self.next_block_type, &BlockRot::Rot0, x, y);
    }

    pub fn collides(&self) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if self.probe_active_block_at(x, y) {
                    let bx = self.block_offset.0 + x;
                    let by = self.block_offset.1 + y;
                    match self.cell_value_at_board(bx, by) {
                        CellVal::Free => (/* free */),
                        _ => return true,
                    }
                }
            }
        }
        return false;
    }

    pub fn freeze_block(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
                if self.probe_active_block_at(x, y) {
                    let bx = self.block_offset.0 + x;
                    let by = self.block_offset.1 + y;
                    self.set_call_value(bx, by, self.block_color);
                }
            }
        }
    }

    pub fn next_block(&mut self) {
        self.block_type = self.next_block_type;
        self.block_color = self.next_block_color;
        self.block_rot = BlockRot::Rot0;
        self.block_offset = (BOARD_DIM_X as i32 / 2 - 2, 0);
        self.next_block_type = BlockType::rand();
        self.next_block_color = CellVal::rand_color();
    }

    fn is_row_full(&self, row: i32) -> bool {
        for x in 0..self.width() {
            match self.cell_value_at_board(x, row) {
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

fn probe(block_type: &BlockType, block_rot: &BlockRot, x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    return match block_type {
        BlockType::I | BlockType::O => {
            x < 4 && y < 4 && pattern_4x4(&block_type, &block_rot)[y as usize][x as usize] == 1
        }
        BlockType::T | BlockType::J | BlockType::L | BlockType::S | BlockType::Z => {
            x < 3 && y < 3 && pattern_3x3(&block_type, &block_rot)[y as usize][x as usize] == 1
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
