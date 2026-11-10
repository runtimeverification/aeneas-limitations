extern "C" { fn ext_add(a: u32, b: u32) -> u32; }
pub fn call(a: u32) -> u32 { unsafe { ext_add(a, 1) } }
