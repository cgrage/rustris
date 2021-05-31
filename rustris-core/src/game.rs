use crate::block::Block;
use crate::board::Board;
use crate::common::{Stats, UserInput};

pub struct Game {
    time: u32,
    step_interval: u32,
    pub board: Board,
    pub active_block: Block,
    pub next_block: Block,
    pub stats: Stats,
}

impl Game {
    pub fn new() -> Game {
        return Game {
            time: 0,
            step_interval: 10,
            board: Board::new(),
            active_block: Block::rand(),
            next_block: Block::rand(),
            stats: Stats::new(),
        };
    }

    pub fn run_step(&mut self) {
        self.time += 1;
        if self.time % self.step_interval == 0 {
            self.try_lower_block();
        }
    }

    pub fn handle_input(&mut self, input: &UserInput) {
        match input {
            UserInput::MoveLeft => self.try_move_block_horizontally(-1),
            UserInput::MoveRight => self.try_move_block_horizontally(1),
            UserInput::MoveDown => self.try_lower_block(),
            UserInput::DropDown => self.drop_block(),
            UserInput::RotateLeft => self.try_rotate_block(-1),
            UserInput::RotateRight => self.try_rotate_block(1),
            UserInput::Reset => self.new_game(),
            UserInput::NoInput => (),
        };
    }

    fn try_move_block_horizontally(&mut self, amount: i32) {
        self.active_block.move_horizontally(amount);
        if self.board.collides(&self.active_block) {
            // if we collide: undo action
            self.active_block.move_horizontally(-amount);
            self.active_block.undo_changes(2);
        }
    }

    fn try_rotate_block(&mut self, amount: i32) {
        self.active_block.rotate(amount);
        if self.board.collides(&self.active_block) {
            // if we collide: undo action
            self.active_block.rotate(-amount);
            self.active_block.undo_changes(2);
        }
    }

    fn try_lower_block(&mut self) {
        self.active_block.move_vertically(1);
        if self.board.collides(&self.active_block) {
            // if we collide: undo action and FREEZE block
            self.active_block.move_vertically(-1);
            self.active_block.undo_changes(2);

            self.freeze_block_and_have_next();
        }
    }

    fn drop_block(&mut self) {
        while !self.board.collides(&self.active_block) {
            self.active_block.move_vertically(1);
        }

        self.active_block.move_vertically(-1);
        self.active_block.undo_changes(2);

        // We don't do this right now..
        // self.freeze_block_and_have_next(board);
    }

    fn freeze_block_and_have_next(&mut self) {
        self.board.freeze_block(&self.active_block);

        let row_count = self.board.clear_full_rows();
        if row_count > 0 {
            self.on_rows_cleared(row_count);
        };

        self.active_block.replace(&self.next_block);
        self.next_block.replace_random();
        if self.board.collides(&self.active_block) {
            // our (just placed) new block already collides..
            // player lost the game.
            self.new_game();
        }
    }

    fn new_game(&mut self) {
        self.board.clear();
        self.active_block.replace(&self.next_block);
        self.next_block.replace_random();
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
