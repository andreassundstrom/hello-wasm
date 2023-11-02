import { Player, add_greeting } from "../../pkg/hello_wasm";
import { memory } from "../../pkg/hello_wasm_bg.wasm";

import "./style.css";

const player = Player.new("Andreas");
memory.buffer;
add_greeting("app");
player.draw();
