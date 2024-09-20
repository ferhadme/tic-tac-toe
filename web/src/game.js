import * as wasm from "tictactoe-wasm";

export function start_game() {
    let game = wasm.initialize_game();
    add_event_listeners(game);
}

function play(game, coord_1, coord_2) {
    wasm.play(game, coord_1, coord_2);
    let board = wasm.render_board(game);
    render_board(board);
    let turn = wasm.turn(game);
    change_turn(turn);
    let result = wasm.check_winner(game);
    check_winner(result);
}

function render_board(board) {
    const ttt_elements = document.getElementsByClassName('ttt-elem');
    for (let i = 0; i < ttt_elements.length; i++) {
	ttt_elements[i].textContent = board[i];
    }
}

function change_turn(turn) {
    document.getElementById('info-player').textContent = turn + "'s turn";
}

function check_winner(result) {
    if (result == 0) return;

    const element = document.getElementById('info-player');
    element.textContent = 'Player ' + result + ' won!';
}

function add_event_listeners(game) {
    const ttt_elements = document.getElementsByClassName('ttt-elem');
    for (let i = 0; i < ttt_elements.length; i++) {
	ttt_elements[i].addEventListener('click', add_event_listener_ttt_elem(game, i));
    }
}

function add_event_listener_ttt_elem(game, index) {
    const coord_1 = Math.floor(index / 3);
    const coord_2 = index % 3;
    return () => {
	play(game, coord_1, coord_2);
    };
}

