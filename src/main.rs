use std::io;

fn main()
{
    //Globals
    let mut memory:[u8; 4096] = initialize_memory().unwrap();
    let mut _variable_register:[u8; 16] = [0; 16];
    let mut _stack: Vec<u16> = Vec::new(); // Use pop and push on the stack.
    let mut program_counter: usize = 0x200;
    let mut _index_register: u16;
    let mut _delay_timer: u8;
    let mut _sound_timer: u8;

    loop {
        // Fetch
        let instruction: u16 = get_next_instruction(&mut program_counter, &memory);

        // Decode
        let decoded: (u8, u8, u8, u8) = generate_nibble_tuple(instruction);

        // Execute
        match decoded {
            (0, 0, 0xE, 0) => println!("Clear screen"),
            (0x1, ..) => println!("Jump"),
            (0x6, ..) => println!("Set Register of second nibble to third and fourth."),
            (0x7, ..) => println!("add value to register VX"),
            (0xA, ..) => println!("set index register I"),
            (0xD, ..) => println!("display/draw"),
            _ => println!("Could not parse instruction...")
        }
    }
}


fn initialize_memory() -> io::Result<[u8; 4096]>
{
    let mut temp_memory:[u8; 4096] = [0; 4096];

    // Load fonts
    let fonts:[u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80  // F
    ];

    for (i,j) in (0x050..0x0A0).enumerate() {
        temp_memory[j as usize] = fonts[i];
    }

    // Load in Game to memory
    let bytes = std::fs::read("./assets/BC_test.ch8")?;
    let mut index: u16 = 0x200;

    for byte in &bytes {
        temp_memory[index as usize] = *byte;
        index += 1;
    }

    Ok(temp_memory)
}

fn get_next_instruction(index: &mut usize, memory: &[u8; 4096]) -> u16 {
    let first_byte: u16 = memory[*index] as u16;
    let second_byte: u16 = memory[*index + 1] as u16;
    let result: u16 = (first_byte << 8) + second_byte;

    *index += 2; // Increment Program Counter by two

    result
}

fn generate_nibble_tuple(instruction: u16) -> (u8, u8, u8, u8) {
    let first_nibble = ((instruction & 0xF000) >> 12) as u8;
    let second_nibble = ((instruction & 0x0F00) >> 8) as u8;
    let third_nibble = ((instruction & 0x00F0) >> 4) as u8;
    let fourth_nibble = ((instruction & 0x000F)) as u8;

    (first_nibble, second_nibble, third_nibble, fourth_nibble)
}
