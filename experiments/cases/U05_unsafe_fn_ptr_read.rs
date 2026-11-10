pub unsafe fn rd(p: *const u32) -> u32 { core::ptr::read(p) }
pub fn safe_call(x: &u32) -> u32 { unsafe { rd(x) } }
