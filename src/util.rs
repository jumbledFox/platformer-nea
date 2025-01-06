// For some reason, checking if two strings in rust isn't possible in a const function.
// I've written my own to allow this.
pub const fn const_str_eq(a: &str, b: &str) -> bool {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();

    // The strings aren't equal if they don't have the same number of bytes
    if a_bytes.len() != b_bytes.len() {
        return false;
    }

    // Go through each of the bytes of both strings, if any aren't equal the strings aren't equal
    let mut i = 0;
    while i < a_bytes.len() {
        if a_bytes[i] != b_bytes[i] {
            return false;
        }
        i += 1;
    }
    
    true
}

pub fn approach_target(val: &mut f32, step: f32, target: f32) {
    if *val < target {
        *val = (*val + step).min(target);
    } else if *val > target {
        *val = (*val - step).max(target);
    }
}