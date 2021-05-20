extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use rustris_core::game::Game;
use rustris_core::board::Board;
use rustris_core::common::{Stats, UserInput};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct RustrisGame {
    // game : Game,
    // board : Board,
    // stats : Stats,
}

#[wasm_bindgen]
impl RustrisGame {

    pub fn new() -> RustrisGame {
        return RustrisGame {
            // game : Game::new(),
            // board : Board::new(),
            // stats : Stats::new(),
        }
    }

    pub fn ping(&mut self) {
        alert(&format!("Ping"));
    }

    pub fn run_step(&mut self) {
        // self.game.run_step(&mut self.board, &mut self.stats);
    }
}
