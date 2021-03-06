use jc_chip8::chip8::Chip8;
use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("roms/Pong.ch8").unwrap();
    let mut rom = Vec::new();
    file.read_to_end(&mut rom).unwrap();

    let mut chip8 = Chip8::default();
    chip8.load_rom(&rom);

    for _ in 0..1000 {
        chip8.clock();
    }

    println!("{:?}", chip8.vram());
}
