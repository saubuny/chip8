use macroquad::prelude::*;
use std::fs;

pub const PIXEL_SIZE: i32 = 16;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Chip8 {
    pub opcode: Option<u16>,
    pub ram: [u8; 4096],
    display: [[u8; 32]; 64],
    pub pc: usize,
    index: u16,
    pub stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    registers: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            opcode: None,
            ram: [0; 4096],
            display: [[0; 32]; 64],
            pc: 0,
            index: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            registers: [0; 16],
        }
    }

    // Read bytes and put into ram
    pub fn read_rom(&mut self, path: &str) {
        let rom = fs::read(path).unwrap();
        let index: usize = 0x200;
        for byte in rom {
            // println!("{:#x}", byte);
            self.ram[index] = byte;
        }
    }

    pub fn decode_instruction(&mut self) {
        let instr1 = self.ram[self.pc];
        let instr2 = self.ram[self.pc + 1];
        self.opcode = Some(instr1 as u16 * 256 + instr2 as u16);
        self.pc += 2;

        let n1 = instr1 >> 4;
        let n2 = instr1 & 0xF;
        let n3 = instr2 >> 4;
        let n4 = instr2 & 0xF;

        let nnn: u16 = self.opcode.unwrap() & 0x0FFF;

        match n1 {
            0x0 => {
                if self.opcode.unwrap() == 0x00e0 {
                    self._00e0();
                }
            }
            0x1 => {
                self._1nnn(nnn);
            }
            0x6 => {
                self._6xnn(n2 as usize, instr2 as u8);
            }
            0x7 => {
                self._6xnn(n2 as usize, instr2 as u8);
            }
            0xA => {
                self._annn(nnn);
            }
            0xD => {
                self._dxyn(n2, n3, n4);
            }
            _ => println!(
                "[Warning] Opcode {:#x} not implemented",
                self.opcode.unwrap()
            ),
        }
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer != 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer != 0 {
            self.sound_timer -= 1;
        }
    }

    // Clear Screen
    pub fn _00e0(&mut self) {
        for i in self.display.iter_mut().flat_map(|r| r.iter_mut()) {
            *i = 0;
        }
    }

    // Jump
    pub fn _1nnn(&mut self, nnn: u16) {
        self.pc = nnn as usize;
    }

    // Set Register VX
    pub fn _6xnn(&mut self, x: usize, nn: u8) {
        self.registers[x] = nn;
    }

    // Add to Register VX
    pub fn _7xnn(&mut self, x: usize, nn: u8) {
        self.registers[x] += nn;
    }

    // Set Index Register I
    pub fn _annn(&mut self, nnn: u16) {
        self.index = nnn;
    }

    // Display
    pub fn _dxyn(&mut self, _x: u8, _y: u8, _n: u8) {
        for (i, row) in self.display.iter_mut().enumerate() {
            for (j, col) in row.iter_mut().enumerate() {
                if *col == 1 {
                    draw_rectangle(
                        i as f32 * PIXEL_SIZE as f32,
                        j as f32 * PIXEL_SIZE as f32,
                        PIXEL_SIZE as f32,
                        PIXEL_SIZE as f32,
                        WHITE,
                    )
                }
            }
        }
    }
}
