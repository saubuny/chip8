use chip8::run;

fn main() {
    let _ = pollster::block_on(run());
}
