use std::io;
use std::io::prelude::*;

mod game;
use crate::game::*;

fn main() -> Result<(), ()> {
    let mut game = Game::new();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
	let line = line.unwrap();
	let command = line;

	if command == "quit" {
	    break;
	}

	let coordinates = command.split_whitespace().take(2).collect::<Vec<&str>>();

	if let [c1, c2] = &coordinates[..] {
	    game.play(c1.parse::<usize>().expect(&format!("Invalid coordinates: {}, {}", c1, c2)),
		      c2.parse::<usize>().expect(&format!("Invalid coordinates: {}, {}", c1, c2)))
		.unwrap();
	} else {
	    println!("Pattern couldn't be matched: {:?}", coordinates);
	}
	println!("{}", game);

	match game.check_winner() {
	    Some(player) => {
		println!("{} won!", player);
		return Ok(());
	    },
	    None => {}
	}
	println!();
    }

    return Ok(());
}

// TODO: Add socket for multiplayer functionality
// TODO: Port to WebAssembly
