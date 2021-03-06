use rand::random;

const SCREEN_WIDTH: usize = 64;
const SCREEN_HEIGHT: usize = 32;
const RAM_SIZE: usize = 4 * 1024;
const STACK_SIZE: usize = 16;
const NUM_REGISTERS: usize = 16;
const NUM_KEYS: usize = 16;

pub struct Chip8 {
    vram: [u8; SCREEN_WIDTH * SCREEN_HEIGHT],
    ram: [u8; RAM_SIZE],
    registers: [u8; NUM_REGISTERS],
    stack: [u16; STACK_SIZE],
    i: u16,
    dt: u8,
    st: u8,
    pub pc: u16,
    sp: u8,
    clocks: usize,
    keys: [bool; NUM_KEYS],
}

impl Chip8 {
    pub fn load_rom(&mut self, rom: &[u8]) {
        self.ram[0x200..0x200 + rom.len()].clone_from_slice(rom);
    }

    pub fn clock(&mut self) {
        self.process_opcode();
        if self.clocks % 4 == 0 {
            self.clock_dt();
            self.clock_st();
        }
        self.clocks += 1;
    }

    pub fn vram(&self) -> &[u8; SCREEN_WIDTH * SCREEN_HEIGHT] {
        &self.vram
    }
}

impl Chip8 {
    fn process_opcode(&mut self) {
        let opcode =
            (self.ram[self.pc as usize] as u16) << 8 | self.ram[self.pc as usize + 1] as u16;
        self.pc += 2;

        let id = opcode & 0xF000;
        let addr = opcode & 0x0FFF;
        let nibble = (opcode & 0x000F) as u8;
        let x = (opcode >> 8 & 0xF) as usize;
        let y = (opcode >> 4 & 0xF) as usize;
        let byte = (opcode & 0x00FF) as u8;

        match id {
            0x0000 => match byte {
                0xE0 => self.vram = [0u8; SCREEN_WIDTH * SCREEN_HEIGHT],
                0xEE => self.return_subroutine(),
                _ => self.unknown_opcode(opcode),
            },
            0x1000 => self.pc = addr,
            0x2000 => self.call_subroutine(addr),
            0x3000 => self.skip_if(self.registers[x] == byte),
            0x4000 => self.skip_if(self.registers[x] != byte),
            0x5000 => self.skip_if(self.registers[x] == self.registers[y]),
            0x6000 => self.registers[x] = byte,
            0x7000 => self.registers[x] += byte,
            0x8000 => match nibble {
                0x0 => self.registers[x] = self.registers[y],
                0x1 => self.registers[x] |= self.registers[y],
                0x2 => self.registers[x] &= self.registers[y],
                0x3 => self.registers[x] ^= self.registers[y],
                0x4 => self.add_with_carry(x, y),
                0x5 => self.registers[x] = self.sub_not_borrow(x, y),
                0x6 => self.shift_right(x),
                0x7 => self.registers[x] = self.sub_not_borrow(y, x),
                0xE => self.shift_left(x),
                _ => self.unknown_opcode(opcode),
            },
            0x9000 => self.skip_if(self.registers[x] != self.registers[y]),
            0xA000 => self.i = addr,
            0xB000 => self.pc = addr + self.registers[0] as u16,
            0xC000 => self.registers[x] = byte & random::<u8>(),
            0xD000 => {
                self.registers[0xF] = self.draw_sprite(
                    self.registers[x] as usize,
                    self.registers[y] as usize,
                    nibble as usize,
                ) as u8
            }
            0xE000 => match byte {
                0x9E => self.skip_if(self.keys[self.registers[x] as usize]),
                0xA1 => self.skip_if(!self.keys[self.registers[x] as usize]),
                _ => self.unknown_opcode(opcode),
            },
            0xF000 => match byte {
                0x07 => self.registers[x] = self.dt,
                0x0A => self.pause_if(self.keys.iter().all(|k| !k)),
                0x15 => self.dt = self.registers[x],
                0x18 => self.st = self.registers[x],
                0x1E => self.i += self.registers[x] as u16,
                0x29 => self.i = self.registers[x] as u16 * 5,
                0x33 => self.store_bcd(x),
                0x55 => self.ram[self.i as usize..self.i as usize + x]
                    .clone_from_slice(&self.registers[0..x]),
                0x65 => self.registers[0..x]
                    .clone_from_slice(&self.ram[self.i as usize..self.i as usize + x]),
                _ => self.unknown_opcode(opcode),
            },
            _ => self.unknown_opcode(opcode),
        }
    }

    fn add_with_carry(&mut self, x: usize, y: usize) {
        let (sum, overflow) = self.registers[x].overflowing_add(self.registers[y]);
        self.registers[0xF] = overflow as u8;
        self.registers[x] = sum;
    }

    fn sub_not_borrow(&mut self, x: usize, y: usize) -> u8 {
        self.registers[0xF] = (self.registers[x] <= self.registers[y]) as u8;
        self.registers[x].saturating_sub(self.registers[y])
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

    fn skip_if(&mut self, skip: bool) {
        self.pc += if skip { 2 } else { 0 };
    }

    fn pause_if(&mut self, pause: bool) {
        self.pc -= if pause { 2 } else { 0 };
    }

    fn clock_dt(&mut self) {
        self.dt = self.dt.saturating_sub(1);
    }

    fn clock_st(&mut self) {
        if self.st > 0 {
            // todo!("raise beep request");
            self.st -= 1;
        }
    }

    fn draw_sprite(&mut self, x0: usize, y0: usize, height: usize) -> bool {
        let mut collision = false;
        for y in 0..height {
            let sprite = self.ram[self.i as usize + y];
            for x in 0..8 {
                let addr = (y0 + y) * SCREEN_WIDTH + x0 + x;
                let prev = self.vram[addr];
                self.vram[addr] ^= (sprite & (0x80 >> x)) >> (7 - x);
                collision = self.vram[addr] == 0 && prev == 1;
            }
        }
        collision
    }

    fn load_font(&mut self) {
        self.ram[..16 * 5].clone_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]);
    }

    fn unknown_opcode(&self, opcode: u16) {
        println!("unknown opcode 0x{:04X}", opcode)
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        let mut chip8 = Chip8 {
            vram: [0u8; SCREEN_WIDTH * SCREEN_HEIGHT],
            ram: [0u8; RAM_SIZE],
            registers: [0u8; NUM_REGISTERS],
            stack: [0u16; STACK_SIZE],
            i: 0,
            dt: 0,
            st: 0,
            pc: 0x200,
            sp: 0,
            clocks: 0,
            keys: [false; NUM_KEYS],
        };
        chip8.load_font();
        chip8
    }
}
