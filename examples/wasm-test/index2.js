import { Universe, Cell } from "wasm-test";

const pre = document.querySelector('#game-of-life-canvas');
const universe = Universe.new();

/*const renderLoop = () => {
  pre.textContent = universe.render();
  universe.tick();

  requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);
*/

setInterval(() => {
  pre.textContent = universe.render();
  universe.tick();
},1);
