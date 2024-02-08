use macroquad::prelude::*;

pub const PIXEL_SIZE: i32 = 16;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Chip8 {
    pub ram: [u8; 4096],
    display: [[u8; 32]; 64],
    pub pc: usize,
    index: u16,
    stack: Vec<u16>,
    pub delay_timer: u8,
    pub sound_timer: u8,
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8 {
            ram: [0; 4096],
            display: [[0; 32]; 64],
            pc: 0,
            index: 0,
            stack: vec![],
            delay_timer: 0,
            sound_timer: 0,
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,
        }
    }

    pub fn read_rom(&mut self) {
        todo!()
    }

    pub fn decode_instruction(&mut self) {
        let instr1 = self.ram[self.pc];
        let instr2 = self.ram[self.pc + 1];

        self.pc += 2;

        // I dont know how this bit manipulation works, I'll look into it later
        // instead of stealing off of stack overflow.
        let nibble1 = instr1 >> 4;
        let nibble2 = instr1 & 0xF;
        let nibble3 = instr2 >> 4;
        let nibble4 = instr2 & 0xF;

        // Combine nibbles 2-4 for a 12-bit number
        let nnn: u16 = ((nibble2 << 8) | (nibble3) | (nibble4 << 4)).into();

        match nibble1 {
            0x0 => {
                self._00e0();
            }
            0x1 => {}
            0x6 => {}
            0x7 => {}
            0xA => {}
            0xD => {
                self._dxyn();
            }
            _ => (),
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

    // Set Register VX (pass the register in directly)
    pub fn _6xnn(&mut self, x: &mut u8, nn: u8) {
        *x = nn;
    }

    // Add to Register VX
    pub fn _7xnn(&mut self, x: &mut u8, nn: u8) {
        *x += nn;
    }

    // Set Index Register I
    pub fn _annn(&mut self, nnn: u16) {
        self.index = nnn;
    }

    // Display
    pub fn _dxyn(&mut self) {
        clear_background(BLACK);

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
