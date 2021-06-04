use rand::Rng;

const BOARD_DIM_X: usize = 10;
const BOARD_DIM_Y: usize = 20;

pub struct Board {
    pub cells: [[CellVal; BOARD_DIM_X]; BOARD_DIM_Y],
    pub change_count: i32,
}

impl Board {
    pub fn new() -> Board {
        return Board {
            cells: [[CellVal::Free; BOARD_DIM_X]; BOARD_DIM_Y],
            change_count: 0,
        };
    }

    pub fn at(&self, x: i32, y: i32) -> CellVal {
        return self.cells[y as usize][x as usize];
    }

    pub fn set(&mut self, x: i32, y: i32, val: CellVal) {
        return self.cells[y as usize][x as usize] = val;
    }

    pub fn width(&self) -> i32 {
        return self.cells[0].len() as i32;
    }

    pub fn height(&self) -> i32 {
        return self.cells.len() as i32;
    }

    pub fn inc_change_count(&mut self) {
        self.change_count += 1;
    }
}

const TETROMINO_SIZE: usize = 4;

pub struct Tetromino {
    pub cells: [[CellVal; TETROMINO_SIZE]; TETROMINO_SIZE],
    pub offset: (i32, i32),
    pub change_count: i32,
}

impl Tetromino {
    pub fn new() -> Tetromino {
        return Tetromino {
            cells: [[CellVal::Free; TETROMINO_SIZE]; TETROMINO_SIZE],
            offset: (0, 0),
            change_count: 0,
        };
    }

    pub fn at(&self, x: i32, y: i32) -> CellVal {
        return self.cells[y as usize][x as usize];
    }

    pub fn set(&mut self, x: i32, y: i32, val: CellVal) {
        return self.cells[y as usize][x as usize] = val;
    }

    pub fn width(&self) -> i32 {
        return self.cells[0].len() as i32;
    }

    pub fn height(&self) -> i32 {
        return self.cells.len() as i32;
    }

    pub fn inc_change_count(&mut self) {
        self.change_count += 1;
    }
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

#[derive(Copy, Clone)]
pub enum CellVal {
    Free,
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

pub enum UserInput {
    MoveLeft,
    MoveRight,
    MoveDown,
    DropDown,
    RotateLeft,
    RotateRight,
    Reset,
    NoInput,
}
