# Chip-8 emulator

Chip-8 emulator written in Rust 🦀 and compilable to WASM 🕸. 

Below is a web demo using this crate. [Play here!](https://joao-conde.github.io/jc-chip8)

![chip8-web](https://user-images.githubusercontent.com/16060539/135887650-2d98f22f-cdbc-4356-bee4-d1b75d2ec3e6.gif)

# Running

See the [`bin`](./bin) folder.

# API Reference

Use the `Chip8` struct to create an emulator instance and interact with it using the following API:

```rust
pub fn new() -> Chip8;
pub fn load_rom(&mut self, rom: &[u8]);
pub fn reset(&mut self);
pub fn clock(&mut self);
pub fn clock_dt(&mut self);
pub fn clock_st(&mut self);
pub fn pixels(&self) -> Vec<u8>;
pub fn beep(&self) -> bool;
pub fn key_press(&mut self, key: u8);
pub fn key_lift(&mut self, key: u8);
```

Basic usage:

```rust
use jc_chip8::chip8::{Chip8, SCREEN_PIXEL_HEIGHT, SCREEN_PIXEL_WIDTH};

let mut chip8 = Chip8::new();
chip8.load_rom(&rom);

loop {
  chip8.clock();
  chip8.clock_dt();
  chip8.clock_st();

  // Your draw code
  let pixels = chip8.pixels();
  ...
  
  // Your event processing
  match event {
    ... => chip8.key_press(0x01)
    ... => chip8.key_press(0x0F)
    ... => chip8.key_lift(0x0A)
    ... => chip8.key_lift(0x0F)
    ...
  }
  
  // Your sound code
  if chip8.beep() {
    ...
  }
}
```
