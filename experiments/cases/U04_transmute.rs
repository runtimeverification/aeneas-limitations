pub fn t(x: u32) -> [u8; 4] { unsafe { core::mem::transmute(x) } }
