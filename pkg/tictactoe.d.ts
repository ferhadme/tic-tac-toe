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
* @returns {boolean}
*/
export function play(game: Game, c1: number, c2: number): boolean;
/**
* @param {Game} game
* @returns {number}
*/
export function check_winner(game: Game): number;
/**
*/
export class Game {
  free(): void;
}
