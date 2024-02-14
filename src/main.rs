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

#[allow(dead_code)]
enum State {
    Menu,
    Game,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = State::Menu;
    let mut chip8 = Chip8::new();

    // Runs at 60fps by default
    loop {
        match state {
            State::Menu => {
                // TODO: Create a menu for selecting roms
                // Exit back to menu with Esc key
                chip8.read_rom("roms/ibm_logo.ch8");
                state = State::Game;
            }
            State::Game => {
                chip8.update_timers();

                // ~8-12 instructions per frame
                // for _ in 0..8 {
                chip8.decode_instruction();

                // Actually draw to screen
                for (i, row) in chip8.display.iter_mut().enumerate() {
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
                // }
            }
        }

        next_frame().await;
    }
}

// X: The second nibble. Used to look up one of the 16 registers (VX) from V0 through VF.
// Y: The third nibble. Also used to look up one of the 16 registers (VY) from V0 through VF.
// N: The fourth nibble. A 4-bit number.
// NN: The second byte (third and fourth nibbles). An 8-bit immediate number.
// NNN: The second, third and fourth nibbles. A 12-bit immediate memory address.
