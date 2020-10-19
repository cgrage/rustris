use model::Board;
use view::UserInput;

pub struct Game {}

impl Game {
    pub fn new() -> Game {
        return Game {};
    }

    pub fn step(&self, board: &mut Board) {
        if board.time_to_step == 0 {
            self.lower_block(board);
            board.time_to_step = board.step_interval;
        } else {
            board.time_to_step = board.time_to_step - 1;
        }
        board.time = board.time + 1;
    }

    pub fn handle_input(&self, input: &UserInput, board: &mut Board) {
        match input {
            UserInput::MoveLeft => self.move_block_vert(board, -1),
            UserInput::MoveRight => self.move_block_vert(board, 1),
            UserInput::MoveDown => self.lower_block(board),
            UserInput::DropDown => self.drop_block(board),
            UserInput::RotateLeft => self.rotate_block(board, -1),
            UserInput::RotateRight => self.rotate_block(board, 1),
            _ => (),
        };
    }

    fn move_block_vert(&self, board: &mut Board, amount: i32) {
        board.move_block(amount, 0);
        if board.collides() {
            // if we collide: undo action
            board.move_block(-amount, 0);
        }
    }

    fn rotate_block(&self, board: &mut Board, amount: i32) {
        board.rotate_block(amount);
        if board.collides() {
            // if we collide: undo action
            board.rotate_block(-amount);
        }
    }

    fn lower_block(&self, board: &mut Board) {
        board.move_block(0, 1);
        if board.collides() {
            // if we collide: undo action and FREEZE block
            board.move_block(0, -1);
            self.freeze_block_and_have_next(board);
        }
    }

    fn drop_block(&self, board: &mut Board) {
        while !board.collides() {
            board.move_block(0, 1);
        }
        board.move_block(0, -1);
        self.freeze_block_and_have_next(board);
    }

    fn freeze_block_and_have_next(&self, board: &mut Board) {
        board.freeze_block();
        /*let row_count =*/
        board.clear_full_rows();
        board.next_block();
        if board.collides() {
            // our (just placed) new block already collides..
            // player lost the game.
            self.new_game(board);
        }
    }

    fn new_game(&self, board: &mut Board) {
        board.clear();
        board.next_block();
    }
}
