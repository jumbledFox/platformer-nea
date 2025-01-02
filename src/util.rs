// For some reason, checking if two strings in rust isn't possible in a const function.
// I've written my own to allow this.
pub const fn const_str_eq(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    if a_bytes.len() != b_bytes.len() {
        return false;
    }

    let mut i = 0;
    while i < a_bytes.len() {
        if a_bytes[i] != b_bytes[i] {
            return false;
        }
        i += 1;
    }
    
    true
}

// An easy way to find out if a specific bit of a u8 is set
pub const fn is_bit_set_u8(value: u8, bit_position: u8) -> bool {
    (value & (1 << bit_position)) != 0
}