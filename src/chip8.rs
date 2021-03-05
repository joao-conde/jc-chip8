const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4 * 1024;
const STACK_SIZE: usize = 16;

const NUM_REGISTERS: usize = 16;

#[derive(Debug)]
pub struct Chip8 {
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

impl Chip8 {
    fn process_opcode(&mut self) {
        let opcode =
            (self.ram[self.pc as usize] as u16) << 8 | self.ram[self.pc as usize + 1] as u16;
        let id = opcode & 0xF000;
        let addr = opcode & 0x0FFF;
        let nibble = opcode & 0x000F;
        let x = opcode >> 8 & 0xF;
        let y = opcode >> 4 & 0xF;
        let byte = opcode & 0x00FF;
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Chip8 {
            vram: [0u8; SCREEN_WIDTH * SCREEN_HEIGHT],
            ram: [0u8; RAM_SIZE],
            registers: [0u8; NUM_REGISTERS],
            stack: [0u16; STACK_SIZE],
            i: 0,
            dt: 0,
            st: 0,
            pc: 0,
            sp: 0,
        }
    }
}
