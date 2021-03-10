import { default as wasm, Chip8 } from "./jc_chip8.js";

import { beep } from "./sound.js";
import { render } from "./canvas.js";
import { FPS, CLOCK, KEY_MAPPER, getROM } from "./util.js";

(async () => {
    // init wasm module
    await wasm();

    // create Chip8 instance and load ROM
    const rom = await getROM("Pong.ch8");
    const chip8 = new Chip8();
    chip8.loadROM(rom);

    // set clock, render and keyboard handlers
    window.setInterval(() => chip8.clockDT(), 1000 / 60);
    window.setInterval(() => chip8.clockST(), 1000 / 60);
    window.setInterval(() => beep(chip8.beep()), 1000 / 60);
    window.setInterval(() => chip8.clock(), 1000 / CLOCK);
    window.setInterval(() => render(chip8.vram()), 1000 / FPS);
    window.onkeydown = (event) => chip8.keyPress(KEY_MAPPER[event.key]);
    window.onkeyup = (event) => chip8.keyLift(KEY_MAPPER[event.key]);
})();
