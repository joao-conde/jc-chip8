import { default as wasm, Chip8 } from "./jc_chip8.js";

const FPS = 144; // FPS
const CLOCK = 800; // Hz
const PIXEL_SET_COLOR = 0xFFFFFFFFF; // white
const PIXEL_UNSET_COLOR = 0x000000FF; // black

const canvas = document.createElement("canvas");
canvas.width = 64;
canvas.height = 32;
const ctx = canvas.getContext("2d");

const scaledCanvas = document.querySelector("canvas#scaled");
const scaledCtx = scaledCanvas.getContext("2d");
scaledCtx.scale(scaledCanvas.width / canvas.width, scaledCanvas.height / canvas.height);
scaledCtx.imageSmoothingEnabled = false

const image = ctx.createImageData(canvas.width, canvas.height);
const videoBuff = new DataView(image.data.buffer);

(async () => {
    await wasm();
    const rom = await getROM("Pong.ch8");
    const chip8 = new Chip8();
    chip8.load_rom(rom);
    window.setInterval(() => chip8.clock(), 1000 / CLOCK);
    window.setInterval(() => render(chip8.vram()), 1000 / FPS);
})();

async function getROM(rom) {
    const response = await window.fetch(`roms/${rom}`);
    const arrayBuffer = await response.arrayBuffer();
    return new Uint8Array(arrayBuffer);
}

function render(pixels) {
    for (let i = 0; i < pixels.length; i++)
        videoBuff.setUint32(i * 4, pixels[i] ? PIXEL_SET_COLOR : PIXEL_UNSET_COLOR);
    ctx.putImageData(image, 0, 0);
    scaledCtx.drawImage(canvas, 0, 0);
}
