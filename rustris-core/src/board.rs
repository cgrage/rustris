use crate::block::Block;
use crate::common::CellVal;

const BOARD_DIM_X: usize = 10;
const BOARD_DIM_Y: usize = 20;

pub struct Board {
    cells: [[CellVal; BOARD_DIM_X]; BOARD_DIM_Y],
}

impl Board {
    pub fn new() -> Board {
        return Board {
            cells: [[CellVal::Free; BOARD_DIM_X]; BOARD_DIM_Y],
        };
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

    pub fn at(&self, x: i32, y: i32) -> &CellVal {
        if x < 0 || x >= BOARD_DIM_X as i32 || y < 0 || y >= BOARD_DIM_Y as i32 {
            return &CellVal::OutOfBoard;
        } else {
            return &self.cells[y as usize][x as usize];
        }
    }

    fn set_call_value(&mut self, x: i32, y: i32, value: &CellVal) {
        self.cells[y as usize][x as usize] = *value;
    }

    pub fn collides(&self, block: &Block) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if block.probe(x, y) {
                    let (bx, by) = block.pos_to_board_pos(x, y);
                    match self.at(bx, by) {
                        CellVal::Free => (/* free */),
                        _ => return true,
                    }
                }
            }
        }
        return false;
    }

    pub fn freeze_block(&mut self, block: &Block) {
        for y in 0..4 {
            for x in 0..4 {
                if block.probe(x, y) {
                    let (bx, by) = block.pos_to_board_pos(x, y);
                    self.set_call_value(bx, by, block.color());
                }
            }
        }
    }

    fn is_row_full(&self, row: i32) -> bool {
        for x in 0..self.width() {
            match self.at(x, row) {
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
