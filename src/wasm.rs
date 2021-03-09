use crate::chip8;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Chip8 {
    chip8: Mutex<chip8::Chip8>,
}

#[wasm_bindgen]
impl Chip8 {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Chip8 {
        Chip8 {
            chip8: Mutex::new(chip8::Chip8::new()),
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.chip8.lock().unwrap().load_rom(rom);
    }

    pub fn clock(&mut self) {
        self.chip8.lock().unwrap().clock();
    }

    pub fn vram(&self) -> Vec<u8> {
        self.chip8.lock().unwrap().vram()
    }
}
