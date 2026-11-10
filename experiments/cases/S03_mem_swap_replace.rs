pub fn s(a: &mut u32, b: &mut u32) { core::mem::swap(a, b) }
pub fn r(a: &mut u32) -> u32 { core::mem::replace(a, 0) }
pub fn t(v: &mut Vec<u32>) -> Vec<u32> { core::mem::take(v) }
