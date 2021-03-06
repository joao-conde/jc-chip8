use jc_chip8::chip8;
use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("demo/roms/test/BC_test.ch8").unwrap();
    let mut rom = Vec::new();
    file.read_to_end(&mut rom).unwrap();

    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom(&rom);

    for _ in 0..1000000 {
        chip8.clock();
    }

    let vram = chip8.vram();
    for y in 0..chip8::SCREEN_HEIGHT {
        for x in 0..chip8::SCREEN_WIDTH {
            print!("{}", vram[y * chip8::SCREEN_WIDTH + x])
        }
        println!()
    }
}
