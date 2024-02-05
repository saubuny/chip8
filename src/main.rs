use chip8::arch::*;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "chip8".into(),
        window_resizable: false,
        window_width: PIXEL_SIZE * 64,
        window_height: PIXEL_SIZE * 32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut chip8 = Chip8::new();

    loop {
        // Apparently runs at 60fps by default
        // println!("{}", get_fps());

        // Update timers
        if chip8.delay_timer != 0 {
            chip8.delay_timer -= 1;
        }

        if chip8.sound_timer != 0 {
            chip8.sound_timer -= 1;
        }

        // ~8-12 instructions per frame
        for _ in 0..8 {
            // Fetch Instruction
            let instr1 = chip8.ram[chip8.pc];
            let instr2 = chip8.ram[chip8.pc + 1];

            chip8.pc += 2;

            // I dont know how this bit manipulation works, I'll look into it later
            // instead of stealing off of stack overflow.
            let nibble1 = instr1 >> 4;
            let nibble2 = instr1 & 0xF;
            let nibble3 = instr2 >> 4;
            let nibble4 = instr2 & 0xF;

            // Combine nibbles 2-4 for a 12-bit number
            let nnn: u16 = ((nibble1 << 8) | (nibble2) | (nibble3 << 4)).into();

            // Decode and run
            match nibble1 {
                0x0 => {
                    chip8._00e0();
                }
                0x1 => {}
                0x6 => {}
                0x7 => {}
                0xA => {}
                0xD => {
                    chip8._dxyn();
                }
                _ => (),
            }
        }

        chip8._dxyn();

        next_frame().await;
    }
}
