pub fn rp(x: &mut u32) -> u32 { let p = x as *mut u32; unsafe { *p = 3; *p } }
