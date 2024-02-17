use rand::random;

use crate::{font::*, stack::*};

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

const RAM_SIZE: usize = 4096;
const REG_COUNT: usize = 16;
const NUM_KEYS: usize = 16;

const START_ADDR: u16 = 0x200;

pub struct Emu {
    pc: u16,
    ram: [u8; RAM_SIZE],
    screen: [bool; SCREEN_WIDTH * SCREEN_HEIGHT],
    v_reg: [u8; REG_COUNT],
    i_reg: u16,
    stack: Stack,
    keys: [bool; NUM_KEYS],
    dt: u8,
    st: u8,
}

impl Emu {
    pub fn new() -> Self {
        let mut emu = Self {
            pc: START_ADDR,
            ram: [0; RAM_SIZE],
            screen: [false; SCREEN_WIDTH * SCREEN_HEIGHT],
            v_reg: [0; REG_COUNT],
            i_reg: 0,
            stack: Stack {
                data: [0; STACK_SIZE],
                sp: 0,
            },
            keys: [false; NUM_KEYS],
            dt: 0,
            st: 0,
        };

        emu.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);

        emu
    }

    pub fn reset(&mut self) {
        self.pc = START_ADDR;
        self.ram = [0; RAM_SIZE];
        self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.v_reg = [0; REG_COUNT];
        self.i_reg = 0;
        self.stack.sp = 0;
        self.stack.data = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.dt = 0;
        self.st = 0;
        self.ram[..FONTSET_SIZE].copy_from_slice(&FONTSET);
    }

    pub fn tick(&mut self) {
        let opcode = self.fetch();
    }

    fn fetch(&mut self) -> u16 {
        let higher_byte = self.ram[self.stack.sp as usize] as u16;
        let lower_byte = self.ram[self.stack.sp as usize] as u16;
        let opcode = (higher_byte << 8) | lower_byte;
        self.pc += 2;
        opcode
    }

    fn execute(&mut self, op: u16) {
        let n1 = (op & 0xF000) >> 12;
        let n2 = (op & 0x0F00) >> 8;
        let n3 = (op & 0x00F0) >> 4;
        let n4 = op & 0x000F;

        let x = n2 as usize;
        let y = n3 as usize;

        let nnn = op & 0x0FFF;
        let nn = (op & 0xFF) as u8;

        match (n1, n2, n3, n4) {
            // NOP
            (0, 0, 0, 0) => return,

            // Clear Screen
            (0, 0, 0xE, 0) => self.screen = [false; SCREEN_WIDTH * SCREEN_HEIGHT],

            // Return
            (0, 0, 0xE, 0xE) => {
                let ret_addr = self.stack.pop();
                self.pc = ret_addr;
            }

            // Jump
            (1, _, _, _) => self.pc = nnn,

            // Call Subroutine
            (2, _, _, _) => {
                self.stack.push(self.pc);
                self.pc = nnn;
            }

            // Skip if v[x] == nn
            (3, _, _, _) => {
                if self.v_reg[x] == nn {
                    self.pc += 2;
                }
            }

            // Skip if v[x] != nn
            (4, _, _, _) => {
                if self.v_reg[x] != nn {
                    self.pc += 2;
                }
            }

            // Skip if v[x] == v[y]
            (5, _, _, 0) => {
                if self.v_reg[x] == self.v_reg[y] {
                    self.pc += 2;
                }
            }

            // v[x] = nn
            (6, _, _, _) => self.v_reg[x] = nn,

            // v[x] += nn
            (7, _, _, _) => self.v_reg[x] = self.v_reg[x].wrapping_add(nn),

            // Bitwise operations
            (8, _, _, 1) => self.v_reg[x] |= self.v_reg[y],
            (8, _, _, 2) => self.v_reg[x] &= self.v_reg[y],
            (8, _, _, 3) => self.v_reg[x] ^= self.v_reg[y],

            // v[x] += v[y]
            (8, _, _, 4) => {
                let (vx, carry) = self.v_reg[x].overflowing_add(self.v_reg[y]);
                let vf = if carry { 1 } else { 0 };

                self.v_reg[x] = vx;
                self.v_reg[0xF] = vf;
            }

            // v[x] -= v[y]
            (8, _, _, 5) => {
                let (vx, borrow) = self.v_reg[x].overflowing_sub(self.v_reg[y]);
                let vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = vx;
                self.v_reg[0xF] = vf;
            }

            // v[x] >>= 1
            (8, _, _, 6) => {
                let lsb = self.v_reg[x] & 1;
                self.v_reg[x] >>= 1;
                self.v_reg[0xF] = lsb;
            }

            // v[x] = v[y] - v[x], idk why this exists
            (8, _, _, 7) => {
                let (vx, borrow) = self.v_reg[y].overflowing_sub(self.v_reg[x]);
                let vf = if borrow { 0 } else { 1 };

                self.v_reg[x] = vx;
                self.v_reg[0xF] = vf;
            }

            // v[x] <<= 1
            (8, _, _, 0xE) => {
                let msb = (self.v_reg[x] >> 7) & 1;
                self.v_reg[x] <<= 1;
                self.v_reg[0xF] = msb;
            }

            // Skip if v[x] != v[y]
            (9, _, _, 0) => {
                if self.v_reg[x] != self.v_reg[y] {
                    self.pc += 2
                }
            }

            // i = nnn
            (0xA, _, _, _) => self.i_reg = nnn,

            // Jump to v[0] + nnn
            (0xB, _, _, _) => self.pc = (self.v_reg[0] as u16) + nnn,

            // v[x] = rand() & nn
            (0xC, _, _, _) => self.v_reg[x] = random::<u8>() & nn,

            (_, _, _, _) => unimplemented!("[Error] Unsupported opcode: {}", op),
        }
    }

    // Returns true if a beep should be played
    pub fn tick_timers(&mut self) -> bool {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                println!("Reminder to beep...");
                return true;
            }
            self.st -= 1;
        }
        false
    }
}
