import { default as wasm, Chip8 } from "./jc_chip8.js";

wasm().then(() => {
    const chip8 = new Chip8();
    // chip8.load_rom();
    chip8.clock();
    console.log(chip8.vram())
});

