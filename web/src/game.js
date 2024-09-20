import * as wasm from "tictactoe-wasm";

const ttt_elements = document.getElementsByClassName('ttt-elem');
for (let i = 0; i < ttt_elements.length; i++) {
    ttt_elements[i].addEventListener('click', add_event_listener_ttt_elem(i));
}

function add_event_listener_ttt_elem(index) {
    let coord_1 = Math.floor(index / 3);
    let coord_2 = index % 3;
    return () => {
	console.log('Coordinates', coord_1, coord_2);
    };
}

