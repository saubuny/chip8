use chip8::wgpu::run;

fn main() {
    let _ = pollster::block_on(run());
}
