import * as wasm from "../pkg/tictactoe.js";

let ttl_elements = document.getElementsByClassName('ttl-elem');

ttl_elements.addEventListener('click', function (event) {
    console.log(event);
    console.log('Coordinates', c1, c2);
});

