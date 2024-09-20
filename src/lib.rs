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
pub fn play(game: &mut Game, c1: usize, c2: usize) -> bool {
    // TODO: Match Ok, Err for coordinates
    game.play(c1, c2);
    return true;
}

#[wasm_bindgen]
pub fn check_winner(game: &mut Game) -> u8 {
    // TODO: Match Ok(Turn), Err
    game.check_winner();
    return 0;
}
