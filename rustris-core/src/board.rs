use crate::model::Tetromino;
use crate::model::{Board, CellVal};

pub struct BoardController {
    pub board: Board,
}

impl BoardController {
    pub fn new() -> BoardController {
        return BoardController { board: Board::new() };
    }

    pub fn clear(&mut self) {
        for y in 0..self.board.height() {
            for x in 0..self.board.width() {
                self.board.set(x, y, CellVal::Free);
            }
        }
        self.board.inc_change_count();
    }

    pub fn freeze_tetromino(&mut self, tetromino: &Tetromino) {
        for y in 0..4 {
            for x in 0..4 {
                let bx = tetromino.offset.0 + x;
                let by = tetromino.offset.1 + y;
                let in_range = (bx >= 0) && (bx < self.board.width()) && (by >= 0) && (by < self.board.height());
                if !in_range {
                    continue;
                }

                let color = tetromino.at(x, y);
                match color {
                    CellVal::Free => (),
                    _ => self.board.set(bx, by, color),
                };
            }
        }
        self.board.inc_change_count();
    }

    fn is_row_full(&self, row: i32) -> bool {
        for x in 0..self.board.width() {
            match self.board.at(x, row) {
                CellVal::Free => return false,
                _ => (),
            }
        }
        return true;
    }

    fn remove_row(&mut self, row: i32) {
        for y in (0..row).rev() {
            for x in 0..self.board.width() {
                let val = self.board.at(x, y);
                self.board.set(x, y + 1, val);
            }
        }
        for x in 0..self.board.width() {
            self.board.set(x, 0, CellVal::Free);
        }
        self.board.inc_change_count();
    }

    pub fn clear_full_rows(&mut self) -> i32 {
        let mut count = 0;
        for y in 0..self.board.height() {
            if self.is_row_full(y) {
                self.remove_row(y);
                count = count + 1;
            }
        }
        self.board.inc_change_count();
        return count;
    }
}
