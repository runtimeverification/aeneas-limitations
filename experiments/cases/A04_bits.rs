pub fn b(a: u32, b: u32) -> u32 { (a & b) | (a ^ b) | !a }
pub fn sh(a: u32, k: u32) -> u32 { (a << k) >> 1 }
pub fn rot(a: u32) -> u32 { a.rotate_left(3) }
pub fn cnt(a: u64) -> u32 { a.count_ones() }
