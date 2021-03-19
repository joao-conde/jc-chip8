import { Chip8 } from "./jc_chip8.js";

import { beep } from "./sound.js";
import { getROM } from "./roms.js";
import { updateCanvas } from "./canvas.js";

let chip8 = null;

const KEY_MAPPER = {
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
    if (chip8) chip8.key_press(KEY_MAPPER[event.key]);
};

export const onKeyUp = (event) => {
    if (chip8) chip8.key_lift(KEY_MAPPER[event.key]);
};
