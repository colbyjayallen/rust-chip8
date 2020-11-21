
pub fn op_0x6XNN(new_value: u8, register_value: &mut u8) {
    *register_value = new_value;
}

pub fn op_0xANNN(new_value: u16, index_register: &mut u16) {
    *index_register = new_value;
}
