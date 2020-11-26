use std::io;

pub fn initalize_memory() -> [u8; 4096] {
    let result = try_initialize_memory();

    match result {
        Ok(n) => {
            n
        },
        Err(n) => {
            eprintln!("COULD NOT READ ROM INTO MEMORY");
            [0; 4096]
        }
    }
}

fn try_initialize_memory() -> io::Result<[u8; 4096]>
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
    let bytes = std::fs::read("./assets/IBM_Logo.ch8")?; //TODO Currently harcoded in ROM value
    let mut index: u16 = 0x200;

    for byte in &bytes {
        temp_memory[index as usize] = *byte;
        index += 1;
    }

    Ok(temp_memory)
}