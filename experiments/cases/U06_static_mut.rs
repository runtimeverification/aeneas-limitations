pub static K: u32 = 7;
pub const C: u32 = 3;
pub static mut G: u32 = 0;
pub fn k() -> u32 { K + C }
pub fn g() -> u32 { unsafe { G += 1; G } }
