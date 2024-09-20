use wasm_bindgen::prelude::*;

pub mod game;
use crate::game::*;

#[wasm_bindgen]
extern {
}

#[wasm_bindgen]
pub fn initialize_game() -> Game {
    return Game::new();
}

#[wasm_bindgen]
pub fn play(game: &mut Game, c1: usize, c2: usize) {
    let _ = game.play(c1, c2);
}

#[wasm_bindgen]
pub fn check_winner(game: &mut Game) -> u8 {
    return match game.check_winner() {
	Some(Turn::P1) => 1,
	Some(Turn::P2) => 2,
	None => 0
    };
}

#[wasm_bindgen]
pub fn render_board(game: &Game) -> Vec<String> {
    return game.render_board_as_vec();
}

#[wasm_bindgen]
pub fn turn(game: &mut Game) -> String {
    return game.get_turn();
}
