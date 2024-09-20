/* tslint:disable */
/* eslint-disable */
/**
* @returns {Game}
*/
export function initialize_game(): Game;
/**
* @param {Game} game
* @param {number} c1
* @param {number} c2
*/
export function play(game: Game, c1: number, c2: number): void;
/**
* @param {Game} game
* @returns {number}
*/
export function check_winner(game: Game): number;
/**
* @param {Game} game
* @returns {(string)[]}
*/
export function render_board(game: Game): (string)[];
/**
* @param {Game} game
* @returns {string}
*/
export function turn(game: Game): string;
/**
*/
export class Game {
  free(): void;
}
