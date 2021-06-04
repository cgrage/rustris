extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use rustris_core::game::Game;
use rustris_core::model::CellVal;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub struct RustrisGame {
    game: Game,
}

fn to_js_color(color : &CellVal) -> i32 {
    match color {
        CellVal::Free => -1,
        CellVal::Color1 => 1,
        CellVal::Color2 => 2,
        CellVal::Color3 => 3,
        CellVal::Color4 => 4,
        CellVal::Color5 => 5,
        CellVal::Color6 => 6,
        CellVal::Color7 => 7,
        CellVal::Color8 => 8,
    }
}

#[wasm_bindgen]
impl RustrisGame {
    pub fn new() -> RustrisGame {
        console_error_panic_hook::set_once();
        return RustrisGame { game: Game::new() };
    }

    pub fn print_info(&mut self) {
        log("RustrisGame");
    }

    pub fn run_step(&mut self) -> bool {
        let mut cc1 = 0;
        cc1 += self.game.current_board().change_count;
        cc1 += self.game.active_piece().change_count;
        cc1 += self.game.next_piece().change_count;

        self.game.run_step();
        let mut cc2 = 0;
        cc2 += self.game.current_board().change_count;
        cc2 += self.game.active_piece().change_count;
        cc2 += self.game.next_piece().change_count;

        return cc1 != cc2;
    }

    pub fn board_color_at(&self, x: i32, y: i32) -> i32 {
        return to_js_color(&self.game.current_board().at(x, y));
    }

    pub fn active_piece_at(&self, x: i32, y: i32) -> i32 {
        return to_js_color(&self.game.active_piece().at(x, y));
    }

    pub fn active_piece_x(&self) -> i32 {
        return self.game.active_piece().offset.0;
    }

    pub fn active_piece_y(&self) -> i32 {
        return self.game.active_piece().offset.1;
    }

    pub fn next_piece_at(&self, x: i32, y: i32) -> i32 {
        return to_js_color(&self.game.next_piece().at(x, y));
    }
}
