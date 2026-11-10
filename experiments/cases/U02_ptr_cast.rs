pub fn pc(x: &u32) -> u8 { let p = x as *const u32 as *const u8; unsafe { *p } }
pub fn addr(x: &u32) -> usize { x as *const u32 as usize }
