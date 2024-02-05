use macroquad::{prelude::*, rand};

pub const PIXEL_SIZE: i32 = 16;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Chip8 {
    pub ram: [u8; 4096],
    display: [[u8; 32]; 64],
    pub pc: u8,
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
        let mut display = [[0; 32]; 64];
        for i in display.iter_mut().flat_map(|r| r.iter_mut()) {
            *i = rand::gen_range(0, 2);
        }

        Chip8 {
            ram: [0; 4096],
            display,
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

    // Clear Screen
    pub fn _00e0(&mut self) {}

    // Jump
    pub fn _1nnn(&mut self) {}

    // Set Register VX
    pub fn _6xnn(&mut self) {}

    // Add to Register VX
    pub fn _7xnn(&mut self) {}

    // Set Index Register I
    pub fn _annn(&mut self) {}

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
