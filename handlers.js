import { Chip8 } from "./jc_chip8.js";

import { beep } from "./sound.js";
import { getROM } from "./roms.js";
import { KEY_MAPPER } from "./keys.js";
import { updateCanvas } from "./canvas.js";

let chip8 = null;

export const play = async () => {
    const rom = await getROM();
    chip8 = new Chip8();
    chip8.load_rom(rom);
};

export const clock = () => {
    if (chip8) chip8.clock();
};

export const clockDT = () => {
    if (chip8) chip8.clock_dt();
};

export const clockST = () => {
    if (chip8) chip8.clock_st();
};

export const render = () => {
    if (chip8) updateCanvas(chip8.pixels());
};

export const audio = () => {
    if (chip8 && chip8.beep()) beep();
};

export const onKeyDown = (event) => {
    const key = KEY_MAPPER[event.key];
    if (key === undefined) return;
    if (chip8) chip8.key_press(key);
    document.querySelector(`#key-${event.key}`).style.opacity = "1";
};

export const onKeyUp = (event) => {
    const key = KEY_MAPPER[event.key];
    if (key === undefined) return;
    if (chip8) chip8.key_lift(key);
    document.querySelector(`#key-${event.key}`).style.opacity = "0.3";
};
