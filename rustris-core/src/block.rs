use crate::common::CellVal;
use rand::Rng;

pub struct Block {
    block_type: BlockType,
    rotation: BlockRot,
    pos: (i32, i32),
    color: CellVal,
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

impl Block {
    pub fn rand() -> Block {
        let mut block = Block {
            block_type: BlockType::I,
            color: CellVal::Free,
            rotation: BlockRot::Rot0,
            pos: (0, 0),
        };
        block.replace_random();
        return block;
    }

    pub fn probe_value(&self, x: i32, y: i32) -> &CellVal {
        if self.probe(x, y) {
            return &self.color;
        } else {
            return &CellVal::Free;
        }
    }

    pub fn pos_to_board_pos(&self, x: i32, y: i32) -> (i32, i32) {
        return (&self.pos.0 + x, &self.pos.1 + y);
    }

    pub fn color(&self) -> &CellVal {
        return &self.color;
    }

    pub fn move_horizontally(&mut self, x: i32) {
        self.pos = (self.pos.0 + x, self.pos.1);
    }

    pub fn move_vertically(&mut self, y: i32) {
        self.pos = (self.pos.0, self.pos.1 + y);
    }

    pub fn rotate(&mut self, r: i32) {
        match r {
            1 => {
                self.rotation = match self.rotation {
                    BlockRot::Rot0 => BlockRot::Rot1,
                    BlockRot::Rot1 => BlockRot::Rot2,
                    BlockRot::Rot2 => BlockRot::Rot3,
                    BlockRot::Rot3 => BlockRot::Rot0,
                }
            }
            -1 => {
                self.rotation = match self.rotation {
                    BlockRot::Rot0 => BlockRot::Rot3,
                    BlockRot::Rot1 => BlockRot::Rot0,
                    BlockRot::Rot2 => BlockRot::Rot1,
                    BlockRot::Rot3 => BlockRot::Rot2,
                }
            }
            _ => panic!("rotate_block must be called w/ 1 or -1"),
        }
        // self.has_change = true;
    }

    pub fn probe(&self, x: i32, y: i32) -> bool {
        return probe(&self.block_type, &self.rotation, x, y);
    }

    pub fn replace(&mut self, other: &Block) {
        self.block_type = other.block_type;
        self.color = other.color;
        self.rotation = BlockRot::Rot0;
        self.pos = (3, 0);
    }

    pub fn replace_random(&mut self){
        self.block_type = BlockType::rand();
        self.color = CellVal::rand_color();
        self.rotation = BlockRot::Rot0;
        self.pos = (3, 0);
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
