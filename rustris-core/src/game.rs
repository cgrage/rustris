use crate::board::Board;
use crate::common::{Stats, UserInput};

pub struct Game {
    time: u32,
    step_interval: u32,
    has_change: bool,
}

impl Game {
    pub fn new() -> Game {
        return Game {
            time: 0,
            step_interval: 10,
            has_change: false,
        };
    }

    pub fn run_step(&mut self, board: &mut Board, stats: &mut Stats) {
        self.time += 1;
        if self.time % self.step_interval == 0 {
            self.lower_block(board, stats);
        }
    }

    pub fn has_change(&mut self) -> bool {
        let val = self.has_change;
        self.has_change = false;
        return val;
    }

    pub fn handle_input(&mut self, input: &UserInput, board: &mut Board, stats: &mut Stats) {
        match input {
            UserInput::MoveLeft => self.move_block_vert(board, -1),
            UserInput::MoveRight => self.move_block_vert(board, 1),
            UserInput::MoveDown => self.lower_block(board, stats),
            UserInput::DropDown => self.drop_block(board),
            UserInput::RotateLeft => self.rotate_block(board, -1),
            UserInput::RotateRight => self.rotate_block(board, 1),
            UserInput::Reset => self.new_game(board, stats),
            UserInput::NoInput => (),
        };
    }

    fn move_block_vert(&mut self, board: &mut Board, amount: i32) {
        board.move_block(amount, 0);
        if board.collides() {
            // if we collide: undo action
            board.move_block(-amount, 0);
        } else {
            self.has_change = true;
        }
    }

    fn rotate_block(&mut self, board: &mut Board, amount: i32) {
        board.rotate_block(amount);
        if board.collides() {
            // if we collide: undo action
            board.rotate_block(-amount);
        } else {
            self.has_change = true;
        }
    }

    fn lower_block(&mut self, board: &mut Board, stats: &mut Stats) {
        board.move_block(0, 1);
        if board.collides() {
            // if we collide: undo action and FREEZE block
            board.move_block(0, -1);
            self.freeze_block_and_have_next(board, stats);
        }
        self.has_change = true;
    }

    fn drop_block(&mut self, board: &mut Board) {
        while !board.collides() {
            board.move_block(0, 1);
        }
        board.move_block(0, -1);
        // We don't do this right now..
        // self.freeze_block_and_have_next(board);
        self.has_change = true;
    }

    fn freeze_block_and_have_next(&mut self, board: &mut Board, stats: &mut Stats) {
        board.freeze_block();

        let row_count = board.clear_full_rows();
        if row_count > 0 {
            self.on_rows_cleared(row_count, stats)
        };

        board.next_block();
        if board.collides() {
            // our (just placed) new block already collides..
            // player lost the game.
            self.new_game(board, stats);
        }
    }

    fn new_game(&self, board: &mut Board, stats: &mut Stats) {
        board.clear();
        board.next_block();
        stats.reset();
    }

    fn on_rows_cleared(&self, amount: i32, stats: &mut Stats) {
        stats.cleared += amount;
        match amount {
            1 => stats.clr_cmb_1 += 1,
            2 => stats.clr_cmb_2 += 1,
            3 => stats.clr_cmb_3 += 1,
            4 => stats.clr_cmb_4 += 1,
            _ => panic!("Cleared strange mount of lines.."),
        }
    }
}
