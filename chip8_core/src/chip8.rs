use std::fs;

// TODO: Keypad

#[derive(Debug)]
#[allow(dead_code)]
pub struct Chip8 {
    opcode: Option<u16>,
    ram: [u8; 4096],
    pub display: [[u8; 32]; 64],
    pc: usize,
    index: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    registers: [u8; 16],
}

impl Chip8 {
    pub fn new() -> Self {
        // Init font
        let mut ram = [0; 4096];
        let font: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80,
            0xF0, 0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0,
            0x10, 0xF0, 0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90,
            0xF0, 0x90, 0xF0, 0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0,
            0x90, 0xE0, 0x90, 0xE0, 0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0,
            0xF0, 0x80, 0xF0, 0x80, 0xF0, 0xF0, 0x80, 0xF0, 0x80, 0x80,
        ];
        let mut ram_idx = 0x50;
        for font_idx in 0..80 {
            ram[ram_idx] = font[font_idx];
            ram_idx += 1;
        }

        Chip8 {
            opcode: None,
            display: [[0; 32]; 64],
            pc: 0,
            ram,
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
                self._6xnn(n2.into(), instr2.into());
            }
            0x7 => {
                self._6xnn(n2.into(), instr2.into());
            }
            0xA => {
                self._annn(nnn);
            }
            0xD => {
                self._dxyn(n2.into(), n3.into(), n4);
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
    pub fn _dxyn(&mut self, x: usize, y: usize, n: u8) {
        // Prevent wrapping
        let mut x: usize = (self.registers[x] % 64).into();
        let mut y: usize = (self.registers[y] % 32).into();
        self.registers[0xF] = 0;

        for i in 0..n {
            let data = self.ram[self.index as usize + i as usize];

            for bit in 0..8 {
                let bit = data & (1 << (8 - bit));
                if bit != 0 {
                    match self.display[x][y] {
                        0 => self.display[x][y] = 1,
                        1 => {
                            self.display[x][y] = 0;
                            self.registers[0xF] = 1;
                        }
                        _ => (),
                    }
                };
                x += 1;
            }

            y += 1;
        }
    }
}
