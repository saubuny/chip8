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
        // Limit to 60fps, https://github.com/not-fl3/macroquad/issues/380
        let minimum_frame_time = 1. / 60.;
        let frame_time = get_frame_time();
        if frame_time < minimum_frame_time {
            let time_to_sleep = (minimum_frame_time - frame_time) * 1000.;
            std::thread::sleep(std::time::Duration::from_millis(time_to_sleep as u64));
        }

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

            // Decode and run
        }

        chip8._dxyn();

        next_frame().await;
    }
}
