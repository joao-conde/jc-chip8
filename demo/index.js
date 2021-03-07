import { default as wasm, Chip8 } from "./jc_chip8.js";

const pixelSetColor = 0xFFFFFFFF  //white
const pixelUnsetColor = 0x000000FF //black
const canvas = document.querySelector('canvas#main')
const ctx = canvas.getContext('2d');
const image = ctx.createImageData(canvas.width, canvas.height);
const videoBuff = new DataView(image.data.buffer);

(async () => {
    await wasm();

    const rom = await getROM("Pong.ch8");

    const chip8 = new Chip8();
    chip8.load_rom(rom);

    const clockFreq = 500 //Hz
    const fps = 60 //FPS

    const clock = function() {
        this.clock
    }.bind(chip8);

    const draw = function() {
        const pixels = this.vram();
        for (let i = 0, j = 0; i < pixels.length; i++, j += 4) videoBuff.setUint32(j, pixels[i] === 1 ? pixelSetColor : pixelUnsetColor)
        ctx.putImageData(image, 0, 0)
        ctx.drawImage(canvas, 0, 0)
    }.bind(chip8);

    window.setInterval(() => clock, 1000 / clockFreq)
    window.setInterval(() => draw, 1000 / fps)
})();

function getROM(rom) {
    return new Promise(resolve => {
        const request = new XMLHttpRequest();
        request.open("GET", `roms/${rom}`, true);
        request.responseType = 'arraybuffer'
        request.onload = () => resolve(request.response)
        request.send()
    })
}
