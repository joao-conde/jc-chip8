import { default as wasm } from "./jc_chip8.js";

import { listROMs } from "./roms.js";
import { play, clock, clockDT, clockST, render, audio, onKeyDown, onKeyUp } from "./handlers.js";

const FPS = 144;
const CLOCK_HZ = 800;
const CLOCK_DT_HZ = 60;
const CLOCK_ST_HZ = 60;
const SOUND_HZ = 60;

(async () => {
    // init wasm module
    await wasm();

    // set clock, video, audio and keyboard handlers
    window.setInterval(clock, 1000 / CLOCK_HZ);
    window.setInterval(clockDT, 1000 / CLOCK_DT_HZ);
    window.setInterval(clockST, 1000 / CLOCK_ST_HZ);
    window.setInterval(render, 1000 / FPS);
    window.setInterval(audio, 1000 / SOUND_HZ);
    window.onkeydown = onKeyDown;
    window.onkeyup = onKeyUp;

    // play ROM on change
    document.querySelector('#roms').onchange = e => play(e.target.value);

    listROMs();
})();
