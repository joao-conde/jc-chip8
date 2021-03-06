import { default as wasm, Chip8 } from "./jc_chip8.js";

wasm().then(() => {
    const chip8 = new Chip8();
    const rom = getROM("Pong.ch8")
    chip8.load_rom(rom);
    for (let i = 0; i < 10000; i++) chip8.clock();

    const vram = chip8.vram()
    for (let y = 0; y < 32; y++) {
        let str = ""
        for (let x = 0; x < 64; x++) str += vram[y * 64 + x].toString()
        console.log(str)
    }
});

function getROM(rom) {
    return new Promise(resolve => {
        const request = new XMLHttpRequest();
        request.open("GET", `demo/roms/${rom}`, true); 
        request.responseType = 'arraybuffer'
        request.onload = () => {
            const result = request.response; 
            resolve(result)
        }
        request.send()
    })
}
