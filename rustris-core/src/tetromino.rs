use crate::model::Board;
use crate::model::CellVal;
use crate::model::Tetromino;
use rand::Rng;

pub struct TetrominoController {
    t_type: TetrominoType,
    rotation: TetrominoRotation,
    color: CellVal,
    pub tetromino: Tetromino,
}

#[derive(Copy, Clone)]
enum TetrominoType {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

impl TetrominoType {
    pub fn rand() -> TetrominoType {
        let mut rnd = rand::thread_rng();
        return match rnd.gen_range(0, 7) {
            0 => TetrominoType::I,
            1 => TetrominoType::O,
            2 => TetrominoType::T,
            3 => TetrominoType::J,
            4 => TetrominoType::L,
            5 => TetrominoType::S,
            _ => TetrominoType::Z,
        };
    }
}

#[derive(Copy, Clone)]
enum TetrominoRotation {
    Rot0,
    Rot1,
    Rot2,
    Rot3,
}

impl TetrominoController {
    pub fn rand() -> TetrominoController {
        let mut new = TetrominoController {
            t_type: TetrominoType::I,
            color: CellVal::Free,
            rotation: TetrominoRotation::Rot0,
            tetromino: Tetromino::new(),
        };
        new.set_random();
        new.update_tetromino();
        return new;
    }

    pub fn try_move_horizontally(&mut self, x: i32, board: &Board) -> bool {
        return self.try_change(x, 0, 0, board);
    }

    pub fn try_move_vertically(&mut self, y: i32, board: &Board) -> bool {
        return self.try_change(0, y, 0, board);
    }

    pub fn try_rotate(&mut self, r: i32, board: &Board) -> bool {
        return self.try_change(0, 0, r, board);
    }

    fn try_change(&mut self, x_change: i32, y_change: i32, rot_change: i32, board: &Board) -> bool {
        let new_off_x = self.tetromino.offset.0 + x_change;
        let new_off_y = self.tetromino.offset.1 + y_change;
        let new_rotation = next_rotation(self.rotation, rot_change);

        if collides(new_off_x, new_off_y, &self.t_type, &new_rotation, board) {
            return false;
        }

        self.tetromino.offset.0 = new_off_x;
        self.tetromino.offset.1 = new_off_y;
        self.rotation = new_rotation;

        self.tetromino.inc_change_count();
        self.update_tetromino();
        return true;
    }

    pub fn collides(&self, board: &Board) -> bool {
        return collides(
            self.tetromino.offset.0,
            self.tetromino.offset.1,
            &self.t_type,
            &self.rotation,
            board,
        );
    }

    fn update_tetromino(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
                let is_set = tetromino(&self.t_type, &self.rotation)[y as usize][x as usize] == 1;
                self.tetromino.set(
                    x,
                    y,
                    match is_set {
                        true => self.color,
                        false => CellVal::Free,
                    },
                );
            }
        }
    }

    pub fn update_by(&mut self, other: &TetrominoController) {
        self.t_type = other.t_type;
        self.color = other.color;
        self.rotation = TetrominoRotation::Rot0;
        self.tetromino.offset = (3, 0);

        self.tetromino.inc_change_count();
        self.update_tetromino();
    }

    pub fn set_random(&mut self) {
        self.t_type = TetrominoType::rand();
        self.color = CellVal::rand_color();
        self.rotation = TetrominoRotation::Rot0;
        self.tetromino.offset = (3, 0);

        self.tetromino.inc_change_count();
        self.update_tetromino();
    }
}

fn next_rotation(current: TetrominoRotation, rot_change: i32) -> TetrominoRotation {
    match rot_change {
        0 => current,
        1 => match current {
            TetrominoRotation::Rot0 => TetrominoRotation::Rot1,
            TetrominoRotation::Rot1 => TetrominoRotation::Rot2,
            TetrominoRotation::Rot2 => TetrominoRotation::Rot3,
            TetrominoRotation::Rot3 => TetrominoRotation::Rot0,
        },
        -1 => match current {
            TetrominoRotation::Rot0 => TetrominoRotation::Rot3,
            TetrominoRotation::Rot1 => TetrominoRotation::Rot0,
            TetrominoRotation::Rot2 => TetrominoRotation::Rot1,
            TetrominoRotation::Rot3 => TetrominoRotation::Rot2,
        },
        _ => panic!("rotate must be called w/ 1, -1, or 0. Not {}", rot_change),
    }
}

fn collides(off_x: i32, off_y: i32, t_type: &TetrominoType, rot: &TetrominoRotation, board: &Board) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            let is_set = tetromino(&t_type, &rot)[y as usize][x as usize] == 1;
            if is_set {
                let bx = off_x + x;
                let by = off_y + y;
                if bx < 0 || bx >= board.width() {
                    return true;
                }
                if by < 0 || by >= board.height() {
                    return true;
                }
                match board.at(bx, by) {
                    CellVal::Free => (/* free */),
                    _ => return true,
                }
            }
        }
    }
    return false;
}

// from: https://strategywiki.org/wiki/Tetris/Rotation_systems
fn tetromino(b_type: &TetrominoType, b_rot: &TetrominoRotation) -> [[i32; 4]; 4] {
    return match b_type {
        TetrominoType::I => match b_rot {
            TetrominoRotation::Rot0 => [
                [0, 0, 0, 0], //
                [1, 1, 1, 1],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot1 => [
                [0, 0, 1, 0], //
                [0, 0, 1, 0],
                [0, 0, 1, 0],
                [0, 0, 1, 0],
            ],
            TetrominoRotation::Rot2 => [
                [0, 0, 0, 0], //
                [0, 0, 0, 0],
                [1, 1, 1, 1],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot3 => [
                [0, 1, 0, 0], //
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 1, 0, 0],
            ],
        },
        TetrominoType::O => [
            [0, 0, 0, 0], //
            [0, 1, 1, 0],
            [0, 1, 1, 0],
            [0, 0, 0, 0],
        ],
        TetrominoType::T => match b_rot {
            TetrominoRotation::Rot0 => [
                [0, 1, 0, 0], //
                [1, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot1 => [
                [0, 1, 0, 0], //
                [0, 1, 1, 0],
                [0, 1, 0, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot2 => [
                [0, 0, 0, 0], //
                [1, 1, 1, 0],
                [0, 1, 0, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot3 => [
                [0, 1, 0, 0], //
                [1, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 0, 0, 0],
            ],
        },
        TetrominoType::J => match b_rot {
            TetrominoRotation::Rot0 => [
                [1, 0, 0, 0], //
                [1, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot1 => [
                [0, 1, 1, 0], //
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot2 => [
                [0, 0, 0, 0], //
                [1, 1, 1, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot3 => [
                [0, 1, 0, 0], //
                [0, 1, 0, 0],
                [1, 1, 0, 0],
                [0, 0, 0, 0],
            ],
        },
        TetrominoType::L => match b_rot {
            TetrominoRotation::Rot0 => [
                [0, 0, 1, 0], //
                [1, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot1 => [
                [0, 1, 0, 0], //
                [0, 1, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot2 => [
                [0, 0, 0, 0], //
                [1, 1, 1, 0],
                [1, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot3 => [
                [1, 1, 0, 0], //
                [0, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 0, 0, 0],
            ],
        },
        TetrominoType::S => match b_rot {
            TetrominoRotation::Rot0 => [
                [0, 1, 1, 0], //
                [1, 1, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot1 => [
                [0, 1, 0, 0], //
                [0, 1, 1, 0],
                [0, 0, 1, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot2 => [
                [0, 0, 0, 0], //
                [0, 1, 1, 0],
                [1, 1, 0, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot3 => [
                [1, 0, 0, 0], //
                [1, 1, 0, 0],
                [0, 1, 0, 0],
                [0, 0, 0, 0],
            ],
        },
        TetrominoType::Z => match b_rot {
            TetrominoRotation::Rot0 => [
                [1, 1, 0, 0], //
                [0, 1, 1, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot1 => [
                [0, 0, 1, 0], //
                [0, 1, 1, 0],
                [0, 1, 0, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot2 => [
                [0, 0, 0, 0], //
                [1, 1, 0, 0],
                [0, 1, 1, 0],
                [0, 0, 0, 0],
            ],
            TetrominoRotation::Rot3 => [
                [0, 1, 0, 0], //
                [1, 1, 0, 0],
                [1, 0, 0, 0],
                [0, 0, 0, 0],
            ],
        },
    };
}
