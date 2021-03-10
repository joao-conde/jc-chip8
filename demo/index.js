import { default as wasm, Chip8 } from "./jc_chip8.js";
import { beep } from "./sound.js";
import { render } from "./canvas.js";

const FPS = 144; // FPS
const CLOCK = 800; // Hz

const keyMapper = {
    "1": 0x01,
    "2": 0x02,
    "3": 0x03,
    "4": 0x0C,
    "q": 0x04,
    "w": 0x05,
    "e": 0x06,
    "r": 0x0D,
    "a": 0x07,
    "s": 0x08,
    "d": 0x09, 
    "f": 0x0E,
    "z": 0x0A,
    "x": 0x00,
    "c": 0x0B,
    "v": 0x0F
};

(async () => {
    await wasm();
    const rom = await getROM("Pong.ch8");
    const chip8 = new Chip8();
    chip8.loadROM(rom);
    window.setInterval(() => chip8.clockDT(), 1000 / 60);
    window.setInterval(() => chip8.clockST(), 1000 / 60);
    window.setInterval(() => beep(chip8.beep()), 1000 / 60);
    window.setInterval(() => chip8.clock(), 1000 / CLOCK);
    window.setInterval(() => render(chip8.vram()), 1000 / FPS);
    window.onkeydown = (event) => chip8.keyPress(keyMapper[event.key]);
    window.onkeyup = (event) => chip8.keyLift(keyMapper[event.key]);
})();

async function getROM(rom) {
    const response = await window.fetch(`roms/${rom}`);
    const arrayBuffer = await response.arrayBuffer();
    return new Uint8Array(arrayBuffer);
}


