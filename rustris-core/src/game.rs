use crate::model::Tetromino;
use crate::tetromino::TetrominoController;
use crate::board::BoardController;
use crate::model::{Board, Stats, UserInput};

pub struct Game {
    time: u32,
    step_interval: u32,
    board: BoardController,
    active_piece: TetrominoController,
    next_piece: TetrominoController,
    stats: Stats,
}

impl Game {
    pub fn new() -> Game {
        return Game {
            time: 0,
            step_interval: 10,
            board: BoardController::new(),
            active_piece: TetrominoController::rand(),
            next_piece: TetrominoController::rand(),
            stats: Stats::new(),
        };
    }

    pub fn run_step(&mut self) {
        self.time += 1;
        if self.time % self.step_interval == 0 {
            self.try_lower_piece();
        }
    }

    pub fn handle_input(&mut self, input: &UserInput) {
        match input {
            UserInput::MoveLeft => self.try_move_piece_horizontally(-1),
            UserInput::MoveRight => self.try_move_piece_horizontally(1),
            UserInput::MoveDown => self.try_lower_piece(),
            UserInput::DropDown => self.drop_piece(),
            UserInput::RotateLeft => self.try_rotate_piece(-1),
            UserInput::RotateRight => self.try_rotate_piece(1),
            UserInput::Reset => self.new_game(),
            UserInput::NoInput => (),
        };
    }

    pub fn current_board(&self) -> &Board {
        return &self.board.board;
    }

    pub fn active_piece(&self) -> &Tetromino {
        return &self.active_piece.tetromino;
    }

    pub fn next_piece(&self) -> &Tetromino {
        return &self.next_piece.tetromino;
    }

    pub fn stats(&self) -> &Stats {
        return &self.stats;
    }

    fn try_move_piece_horizontally(&mut self, amount: i32) {
        self.active_piece.try_move_horizontally(amount, &self.board.board);
    }

    fn try_rotate_piece(&mut self, amount: i32) {
        self.active_piece.try_rotate(amount, &self.board.board);
    }

    fn try_lower_piece(&mut self) {
        if !self.active_piece.try_move_vertically(1, &self.board.board) {
            self.freeze_piece_and_have_next();
        }
    }

    fn drop_piece(&mut self) {
        while self.active_piece.try_move_vertically(1, &self.board.board) {
            //
        }

        // We don't do this right now..
        // self.freeze_piece_and_have_next(board);
    }

    fn freeze_piece_and_have_next(&mut self) {
        self.board.freeze_tetromino(&self.active_piece.tetromino);

        let row_count = self.board.clear_full_rows();
        if row_count > 0 {
            self.on_rows_cleared(row_count);
        };

        self.active_piece.update_by(&self.next_piece);
        self.next_piece.set_random();

        if self.active_piece.collides(&self.board.board) {
            // our (just placed) new piece already collides..
            // player lost the game.
            self.new_game();
        }
    }

    fn new_game(&mut self) {
        self.board.clear();
        self.active_piece.set_random();
        self.next_piece.set_random();
        self.stats.reset();
    }

    fn on_rows_cleared(&mut self, amount: i32) {
        self.stats.cleared += amount;
        match amount {
            1 => self.stats.clr_cmb_1 += 1,
            2 => self.stats.clr_cmb_2 += 1,
            3 => self.stats.clr_cmb_3 += 1,
            4 => self.stats.clr_cmb_4 += 1,
            _ => panic!("Cleared strange mount of lines.."),
        }
    }
}
