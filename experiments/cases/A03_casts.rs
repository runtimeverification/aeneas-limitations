pub fn c1(x: u32) -> u8 { x as u8 }
pub fn c2(x: i32) -> u64 { x as u64 }
pub fn c3(b: bool) -> u8 { b as u8 }
pub fn c4(x: u8) -> char { x as char }
pub fn c5(x: u16) -> u32 { u32::from(x) }
pub fn c6(x: u32) -> Result<u8, core::num::TryFromIntError> { u8::try_from(x) }
