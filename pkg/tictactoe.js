
import * as wasm from "./tictactoe_bg.wasm";
import { __wbg_set_wasm } from "./tictactoe_bg.js";
__wbg_set_wasm(wasm);
export * from "./tictactoe_bg.js";
