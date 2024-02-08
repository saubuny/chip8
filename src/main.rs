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

    // Runs at 60fps by default
    // loop {
    //     chip8.update_timers();
    //
    //     // ~8-12 instructions per frame
    //     for _ in 0..8 {
    //         chip8.decode_instruction();
    //     }
    //
    //     next_frame().await;
    // }

    chip8.read_rom()
}

// X: The second nibble. Used to look up one of the 16 registers (VX) from V0 through VF.
// Y: The third nibble. Also used to look up one of the 16 registers (VY) from V0 through VF.
// N: The fourth nibble. A 4-bit number.
// NN: The second byte (third and fourth nibbles). An 8-bit immediate number.
// NNN: The second, third and fourth nibbles. A 12-bit immediate memory address.
