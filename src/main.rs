use macroquad::prelude::*;

#[macroquad::main("chip8")]
async fn main() {
    loop {
        clear_background(RED);

        next_frame().await;
    }
}
