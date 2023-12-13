import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas")

let universe = Universe.new()

const loop = () => {
  pre.textContent = universe.render()
  universe.tick()
  requestAnimationFrame(loop)
}

loop()
