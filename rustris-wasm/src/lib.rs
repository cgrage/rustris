extern crate console_error_panic_hook;
extern crate wasm_bindgen;

use rustris_core::game::Game;
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
        self.game.run_step();
        return self.game.has_change();
    }
}
