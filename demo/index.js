import { default as wasm, Chip8 } from "./jc_chip8.js";

const fps = 60; //FPS
const clockFreq = 500; //Hz
const pixelSetColor = 0xffffffff; //white
const pixelUnsetColor = 0x000000ff; //black
const canvas = document.querySelector("canvas#main");
const ctx = canvas.getContext("2d");
const image = ctx.createImageData(canvas.width, canvas.height);
const videoBuff = new DataView(image.data.buffer);

(async () => {
    await wasm();
    const rom = await getROM("Pong.ch8");
    const chip8 = new Chip8();
    chip8.load_rom(rom);
    window.setInterval(cycle.bind(chip8), 10);
})();

async function getROM(rom) {
    const response = await window.fetch(`roms/${rom}`);
    return await response.arrayBuffer();
}

function cycle() {
    this.clock();
    const pixels = this.vram();
    render(pixels);
}

function render(pixels) {
    for (let i = 0, j = 0; i < pixels.length; i++, j += 4)
        videoBuff.setUint32(
            j,
            pixels[i] === 1 ? pixelSetColor : pixelUnsetColor
        );
    ctx.putImageData(image, 0, 0);
    ctx.drawImage(canvas, 0, 0);
}
