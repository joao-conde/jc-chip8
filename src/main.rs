use jc_chip8::chip8;
use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("demo/roms/Pong.ch8").unwrap();
    let mut rom = Vec::new();
    file.read_to_end(&mut rom).unwrap();

    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom(rom);

    for i in 0..1_000_000_000u64 {
        chip8.clock();
        if i % 10000 == 0 {
            let vram = chip8.vram();
            for y in 0..chip8::SCREEN_HEIGHT {
                for x in 0..chip8::SCREEN_WIDTH {
                    print!("{}", vram[y * chip8::SCREEN_WIDTH + x])
                }
                println!()
            }
        }
    }
}
