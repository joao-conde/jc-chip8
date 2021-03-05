const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4 * 1024;
const STACK_SIZE: usize = 16;

const NUM_REGISTERS: usize = 16;

struct Chip8 {
    vram: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    ram: [u8; RAM_SIZE],
    registers: [u8; NUM_REGISTERS],
    stack: [u16; STACK_SIZE],
    i: u16,
    dt: u8,
    st: u8,
    pc: u16,
    sp: u8,
}
