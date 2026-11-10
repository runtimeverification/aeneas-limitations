fn inc(x: u32) -> u32 { x + 1 }
pub fn apply_ptr(f: fn(u32) -> u32, x: u32) -> u32 { f(x) }
pub fn use_ptr() -> u32 { apply_ptr(inc, 1) }
