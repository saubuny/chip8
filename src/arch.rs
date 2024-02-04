pub const D_WIDTH: usize = 64;
pub const D_HEIGHT: usize = 32;

#[derive(Debug)]
#[allow(dead_code)]
pub struct Chip8 {
    ram: [u8; 4096],
    display: [[u8; D_WIDTH]; D_HEIGHT],
    pc: u8,
    index: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
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
