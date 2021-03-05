use rand::random;

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
    fn add_with_carry(&mut self, x: u8, y: u8) {
        let (sum, overflow) =
            self.registers[x as usize].overflowing_add(self.registers[y as usize]);
        self.registers[0xF] = overflow as u8;
        self.registers[x as usize] = sum;
    }

    fn sub_not_borrow(&mut self, x: u8, y: u8) -> u8 {
        self.registers[0xF] = !(self.registers[x as usize] > self.registers[y as usize]) as u8;
        self.registers[x as usize] - self.registers[y as usize]
    }

    fn call_subroutine(&mut self, addr: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = addr;
    }

    fn return_subroutine(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize];
    }

    fn shift_right(&mut self, x: u8) {
        self.registers[0xF] = self.registers[x as usize] & 0x01;
        self.registers[x as usize] >>= 1;
    }

    fn shift_left(&mut self, x: u8) {
        self.registers[0xF] = (self.registers[x as usize] & 0x08) >> 7;
        self.registers[x as usize] <<= 1;
    }

    fn skip_if_equal(&mut self, x: u8, val: u8) {
        if self.registers[x as usize] == val {
            self.pc += 2;
        }
    }

    fn skip_if_not_equal(&mut self, x: u8, val: u8) {
        if self.registers[x as usize] == val {
            self.pc += 2;
        }
    }

    fn process_opcode(&mut self) {
        let opcode =
            (self.ram[self.pc as usize] as u16) << 8 | self.ram[self.pc as usize + 1] as u16;

        self.pc += 2;

        let id = opcode & 0xF000;
        let addr = opcode & 0x0FFF;
        let nibble = (opcode & 0x000F) as u8;
        let x = (opcode >> 8 & 0xF) as u8;
        let y = (opcode >> 4 & 0xF) as u8;
        let byte = (opcode & 0x00FF) as u8;

        match id {
            0x00E0 => self.vram = [0u8; SCREEN_WIDTH * SCREEN_HEIGHT],
            0x00EE => self.return_subroutine(),
            0x1000 => self.pc = addr,
            0x2000 => self.call_subroutine(addr),
            0x3000 => self.skip_if_equal(x, byte),
            0x4000 => self.skip_if_not_equal(x, byte),
            0x5000 => self.skip_if_equal(x, self.registers[y as usize]),
            0x6000 => self.registers[x as usize] = byte,
            0x7000 => self.registers[x as usize] += byte,
            0x8000 => self.registers[x as usize] = self.registers[y as usize],
            0x8001 => self.registers[x as usize] |= self.registers[y as usize],
            0x8002 => self.registers[x as usize] &= self.registers[y as usize],
            0x8003 => self.registers[x as usize] ^= self.registers[y as usize],
            0x8004 => self.add_with_carry(x, y),
            0x8005 => self.registers[x as usize] = self.sub_not_borrow(x, y),
            0x8006 => self.shift_right(x),
            0x8007 => self.registers[x as usize] = self.sub_not_borrow(y, x),
            0x800E => self.shift_left(x),
            0x9000 => self.skip_if_not_equal(x, self.registers[y as usize]),
            0xA000 => self.i = addr,
            0xB000 => self.pc = addr + self.registers[0] as u16,
            0xC000 => self.registers[x as usize] = byte & random::<u8>() & 0xFF,
            _ => panic!("unknown instruction"),
        }
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
