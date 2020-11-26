mod emulator;
use emulator::chip8::Chip8;

fn main() {
    let mut chip8 = Chip8::new();
    chip8.run();
}
