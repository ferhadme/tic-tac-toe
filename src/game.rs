use wasm_bindgen::prelude::*;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Element {
    X,
    O,
    EMPTY
}

impl std::convert::From<Element> for Turn {
    fn from(item: Element) -> Self {
	use Turn::*;
	use Element::*;
	return if item == X { P1 } else { P2 };
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	let element_str = match *self {
	    Element::X => "X",
	    Element::O => "O",
	    Element::EMPTY => ""
	};
	write!(f, "{}", element_str)?;
	return Ok(());
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Turn {
    P1,
    P2
}

impl std::convert::From<Turn> for Element {
    fn from(item: Turn) -> Self {
	use Turn::*;
	use Element::*;
	return if item == P1 { X } else { O };
    }
}

impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	write!(f, "{}", if *self == Turn::P1 { "Player 1" } else { "Player 2" })?;
	return Ok(());
    }
}

#[wasm_bindgen]
pub struct Game {
    board: [[Element; 3]; 3],
    turn: Turn,
    finished: bool
}

impl Game {
    pub fn new() -> Self {
	let board = [[Element::EMPTY; 3]; 3];
	return Self {board: board, turn: Turn::P1, finished: false};
    }

    pub fn play(&mut self, c1: usize, c2: usize) -> Result<(), String>{
	if c1 > 1 && c1 + c2 > 4 {
	    return Err(format!("Invalid coordinates: {}, {}", c1, c2));
	}

	let element = self.turn.into();

	if self.board[c1][c2] != Element::EMPTY || self.finished {
	    return Err(format!("Game finished"));
	}

	self.board[c1][c2] = element;
	self.change_turn();

	return Ok(());
    }

    pub fn check_winner(&mut self) -> Option<Turn> {
	if let Some(turn) = self.linear_scan(true) {
	    self.finished = true;
	    return Some(turn);
	}
	if let Some(turn) = self.linear_scan(false) {
	    self.finished = true;
	    return Some(turn);
	}
	if let Some(turn) = self.diagonal_scan() {
	    self.finished = true;
	    return Some(turn);
	}

	return None;
    }

    pub fn render_board_as_vec(&self) -> Vec<String> {
	let mut board = Vec::new();
	for i in 0..3 {
	    for j in 0..3 {
		board.push(self.board[i][j].to_string());
	    }
	}
	return board;
    }

    pub fn get_turn(&self) -> String {
	return self.turn.to_string();
    }

    fn change_turn(&mut self) {
	use Turn::*;
	self.turn = if self.turn == P1 { P2 } else { P1 };
    }

    fn diagonal_scan(&self) -> Option<Turn> {
	let mut element = self.board[0][0];
	let mut won = true;

	for i in 1..3 {
	    if element == Element::EMPTY || element != self.board[i][i] {
		won = false;
		break;
	    }
	}
	if won {
	    return Some(Turn::from(element));
	}

	element = self.board[0][2];
	won = true;

	for i in 1..3 {
	    if element == Element::EMPTY || element != self.board[i][2 - i] {
		won = false;
		break;
	    }
	}
	if won {
	    return Some(Turn::from(element));
	}

	return None;
    }

    fn linear_scan(&self, horizontal: bool) -> Option<Turn> {
	for i in 0..3 {
	    let element = if horizontal { self.board[i][0] } else { self.board[0][i] };
	    if element == Element::EMPTY {
		break;
	    }

	    let mut won = true;
	    for j in 1..3 {
		let current = if horizontal { self.board[i][j] } else { self.board[j][i] };
		if element != current {
		    won = false;
		    break;
		}
	    }

	    if won {
		return Some(Turn::from(element));
	    }
	}

	return None;
    }
}

impl std::fmt::Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	write!(f, "\nBoard:\n")?;

	for i in 0..3 {
	    for _ in 0..6 {
		write!(f, "-")?;
	    }
	    write!(f, "\n")?;
	    for j in 0..3 {
		match self.board[i][j] {
		    Element::EMPTY => write!(f, " ")?,
		    Element::X => write!(f, "X")?,
		    Element::O => write!(f, "O")?
		};
		write!(f, "|")?;
	    }
	    write!(f, "\n")?;
	}

	for _ in 0..6 {
	    write!(f, "-")?;
	}

	return Ok(());
    }
}
