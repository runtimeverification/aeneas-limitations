pub fn incr_nested(x: &mut &mut u32) { **x += 1; }
pub fn use_nested() -> u32 { let mut a = 0; let mut r = &mut a; incr_nested(&mut r); a }
