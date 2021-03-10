import { default as wasm, Chip8 } from "./jc_chip8.js";

const fps = 60; // FPS
const clockFreq = 500; // Hz
const pixelSetColor = 0xffffffff; // white
const pixelUnsetColor = 0x000000ff; // black
const canvas = document.querySelector("canvas#main");
const ctx = canvas.getContext("2d");
const image = ctx.createImageData(canvas.width, canvas.height);
const videoBuff = new DataView(image.data.buffer);

(async () => {
    await wasm();
    const rom = await getROM("Pong.ch8");
    const chip8 = new Chip8();
    chip8.load_rom(rom);
    window.setInterval(() => chip8.clock(), 1000 / clockFreq);
    window.setInterval(() => render(chip8.vram()), 1000 / fps);
})();

async function getROM(rom) {
    const response = await window.fetch(`roms/${rom}`);
    const arrayBuffer = await response.arrayBuffer();
    return new Uint8Array(arrayBuffer);
}

function render(pixels) {
    for (let i = 0; i < pixels.length; i++) videoBuff.setUint32(i * 4, pixels[i] ? pixelSetColor : pixelUnsetColor);
    ctx.putImageData(image, 0, 0);
    ctx.drawImage(canvas, 0, 0);
}
