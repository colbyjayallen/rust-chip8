struct Chip8 {
    memory:[u8; 4096],
    variable_register:[u8; 16],
    stack: Vec<u16>,
    program_counter: usize,
    index_register: u16,
    delay_timer: u8,
    sound_timer: u8
}

impl Chip8 {
    pub fn run() {
        
    }
}
