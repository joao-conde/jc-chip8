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

        match opcode {
            0x00E0 => self.vram = [0u8; SCREEN_WIDTH * SCREEN_HEIGHT],
            0x00EE => self.return_subroutine(),
            0x1000 => self.pc = addr,
            0x2000 => self.call_subroutine(addr),
            0x3000 => self.skip_if_equal(x as usize, byte),
            0x4000 => self.skip_if_not_equal(x as usize, byte),
            0x5000 => self.skip_if_equal(x as usize, self.registers[y as usize]),
            0x6000 => self.registers[x as usize] = byte,
            0x7000 => self.registers[x as usize] += byte,
            0x8000 => self.registers[x as usize] = self.registers[y as usize],
            0x8001 => self.registers[x as usize] |= self.registers[y as usize],
            0x8002 => self.registers[x as usize] &= self.registers[y as usize],
            0x8003 => self.registers[x as usize] ^= self.registers[y as usize],
            0x8004 => self.add_with_carry(x as usize, y as usize),
            0x8005 => self.registers[x as usize] = self.sub_not_borrow(x as usize, y as usize),
            0x8006 => self.shift_right(x as usize),
            0x8007 => self.registers[x as usize] = self.sub_not_borrow(y as usize, x as usize),
            0x800E => self.shift_left(x as usize),
            0x9000 => self.skip_if_not_equal(x as usize, self.registers[y as usize]),
            0xA000 => self.i = addr,
            0xB000 => self.pc = addr + self.registers[0] as u16,
            0xC000 => self.registers[x as usize] = byte & random::<u8>(),
            0xD000 => {
                self.registers[0xF] = self.draw_sprite(
                    self.registers[x as usize] as usize,
                    self.registers[y as usize] as usize,
                    nibble as usize,
                ) as u8
            }
            0xE09E => todo!("keyboard"),
            0xE0A1 => todo!("keyboard"),
            0xF007 => self.registers[x as usize] = self.dt,
            0xF00A => todo!("keyboard"),
            0xF015 => self.dt = self.registers[x as usize],
            0xF018 => self.st = self.registers[x as usize],
            0xF01E => self.i += self.registers[x as usize] as u16,
            0xF029 => self.i = self.registers[x as usize] as u16 * 5,
            0xF033 => self.store_bcd(x as usize),
            0xF055 => self.ram[self.i as usize..self.i as usize + x as usize]
                .clone_from_slice(&self.registers[0..x as usize]),
            0xF065 => self.registers[0..x as usize]
                .clone_from_slice(&self.ram[self.i as usize..self.i as usize + x as usize]),
            _ => panic!("unknown instruction"),
        }
    }

    fn add_with_carry(&mut self, x: usize, y: usize) {
        let (sum, overflow) = self.registers[x].overflowing_add(self.registers[y]);
        self.registers[0xF] = overflow as u8;
        self.registers[x] = sum;
    }

    fn sub_not_borrow(&mut self, x: usize, y: usize) -> u8 {
        self.registers[0xF] = (self.registers[x] <= self.registers[y]) as u8;
        self.registers[x] - self.registers[y]
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

    fn shift_right(&mut self, x: usize) {
        self.registers[0xF] = self.registers[x as usize] & 0x01;
        self.registers[x] >>= 1;
    }

    fn shift_left(&mut self, x: usize) {
        self.registers[0xF] = (self.registers[x as usize] & 0x08) >> 7;
        self.registers[x] <<= 1;
    }

    fn store_bcd(&mut self, x: usize) {
        self.ram[self.i as usize] = self.registers[x] / 100;
        self.ram[self.i as usize + 1] = (self.registers[x] / 10) % 10;
        self.ram[self.i as usize + 2] = self.registers[x] % 10;
    }

    fn skip_if_equal(&mut self, x: usize, val: u8) {
        if self.registers[x] == val {
            self.pc += 2;
        }
    }

    fn skip_if_not_equal(&mut self, x: usize, val: u8) {
        if self.registers[x] == val {
            self.pc += 2;
        }
    }

    fn draw_sprite(&mut self, x0: usize, y0: usize, height: usize) -> bool {
        let mut collision = false;
        for y in 0..height {
            let sprite = self.ram[self.i as usize + y];
            for x in 0..8 {
                let addr = (y0 + y) * SCREEN_WIDTH + x0 + x;
                let prev = self.vram[addr];
                self.vram[addr] ^= sprite & (0x80 >> x);
                collision = self.vram[addr] == 0 && prev == 1;
            }
        }
        collision
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
