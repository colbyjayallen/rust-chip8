use std::fs::File;

fn main()
{

    //Globals
    let mut memory:[u8; 4096] = ;
    let mut variable_register:[u8; 16] = [0; 16];
    let mut stack: Vec<u16> = Vec::new(); // Use pop and push on the stack.
    let mut index_register: u16;
    let mut delay_timer: u8;
    let mut sound_timer: u8;

    loop {
        // Fetch
        // Decode
        // Execute
        break;
    }
}

/**
 * Find a way to automate this based on taking input from a CSV
 **/
fn initialize_memory(rom_path: &str) -> [u8; 4096]
{
    let mut temp_memory:[u8; 4096] = [0; 4096];
    let mut index: u8 = 0x050;
    
}

fn load_rom(rom_path: &str) -> [u8; 4096] {
    println!("Loading rom...");

}
