import { Player, add_greeting } from "../../pkg/hello_wasm";
import { memory } from "../../pkg/hello_wasm_bg.wasm";

import "./style.css";
const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const context = canvas.getContext("2d") as CanvasRenderingContext2D;

// Call function that manipulates dom
add_greeting("app");

// Create a "player" and draw it from withing WASM
const player = Player.new("Andreas", 0.0, 0.0);
player.draw("canvas");

/*
   Create another "player", read derectly from WASMs memory
   and draw it in JS
*/
const player_2 = Player.new("Robert", 20, 30);
drawPlayerFromMemory(player_2);

function drawPlayerFromMemory(player_instance: Player) {
  // Get pointer to position struct
  const positioPtr = player_instance.position();
  const position = new Float64Array(memory.buffer, positioPtr, 2);
  context.fillRect(position[0], position[1], 25, 25);

  // Get pointer to name and size of string, read as UInt32 and
  // convert to string
  const namePtr = player_2.name_ptr();
  const nameLength = player_2.name_size();
  const nameBytes = new Uint8Array(memory.buffer, namePtr, nameLength);
  const name = new TextDecoder().decode(nameBytes);

  context.fillText(name, position[0] + 25, position[1] + 25);
}
