pub fn w(a: u32, b: u32) -> u32 { a.wrapping_add(b).wrapping_mul(3) }
pub fn c(a: u32, b: u32) -> Option<u32> { a.checked_add(b) }
pub fn o(a: u8, b: u8) -> (u8, bool) { a.overflowing_add(b) }
pub fn s(a: u8, b: u8) -> u8 { a.saturating_sub(b) }
