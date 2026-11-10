use std::cmp::Ordering;
pub fn mx(a: u32, b: u32) -> u32 { std::cmp::max(a, b) }
pub fn mn(a: u32, b: u32) -> u32 { a.min(b) }
pub fn c(a: u32, b: u32) -> Ordering { a.cmp(&b) }
