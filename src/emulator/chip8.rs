use super::bootstrap;
use super::bit_helpers;

pub struct Chip8 {
    memory:[u8; 4096],
    variable_register:[u8; 16],
    stack: Vec<u16>, // Use pop and push on the stack.
    program_counter: usize,
    index_register: u16,
    delay_timer: u8,
    sound_timer: u8
}

impl Chip8 {
    // Constructor
    pub fn new() -> Chip8 {
        Chip8 {
            memory: bootstrap::initalize_memory(),
            variable_register: [0; 16],
            stack: Vec::new(),
            program_counter: 0x200,
            index_register: 0,
            delay_timer: 0,
            sound_timer: 0
        }
    }

    pub fn run(&mut self) {
        loop {
            let instruction: u16 = Chip8::fetch(self);
            let decoded: (u8, u8, u8, u8) = Chip8::decode(self, instruction);
            Chip8::execute(self, decoded);
        }
    }

    fn fetch(&mut self) -> u16 {
        let first_byte: u16 = self.memory[self.program_counter] as u16;
        let second_byte: u16 = self.memory[self.program_counter + 1] as u16;
        let result: u16 = (first_byte << 8) + second_byte;

        self.program_counter += 2; // Increment Program Counter by two

        result
    }

    fn decode(&mut self, instruction: u16) -> (u8, u8, u8, u8) {
        let first_nibble = ((instruction & 0xF000) >> 12) as u8;
        let second_nibble = ((instruction & 0x0F00) >> 8) as u8;
        let third_nibble = ((instruction & 0x00F0) >> 4) as u8;
        let fourth_nibble = ((instruction & 0x000F)) as u8;

        (first_nibble, second_nibble, third_nibble, fourth_nibble)
    }

    fn execute(&mut self, (a, b, c, d): (u8, u8, u8, u8)) {
        let decoded: (u8, u8, u8, u8) = (a, b, c, d);
        match decoded {
            (0, 0, 0xE, 0) => println!("Clear screen"),
            (0x1, ..) => println!("Jump"),
            (0x6, ..) => {
                let new_value: u8 = bit_helpers::generate_last_byte(decoded);
                Chip8::op_6xnn(self, new_value, decoded.1 as usize);
            },
            (0x7, ..) => {
                let value_to_add: u8 = bit_helpers::generate_last_byte(decoded);
                Chip8::op_7xnn(self, value_to_add, decoded.1 as usize);
            },
            (0xA, ..) => {
                let new_value: u16 = bit_helpers::get_tribble(decoded);
                Chip8::op_annn(self, new_value);
            },
            (0xD, ..) => println!("display/draw"),
            _ => {
                panic!("Could not parse instruction...");
            }
        }
    }

    ///
    /// OPCODE FUNCTIONS
    ///

    fn op_6xnn(&mut self, new_value: u8, register_index: usize) {
        self.variable_register[register_index] = new_value;
    }

    fn op_7xnn(&mut self, value_to_add: u8, register_index: usize) {
        //TODO Add a header guard in case of overflows
        self.variable_register[register_index] = self.variable_register[register_index] + value_to_add;
    }

    fn op_annn(&mut self, new_value: u16) {
        self.index_register = new_value;
    }

}
