pub union U { pub a: u32, pub b: f32 }
pub fn read(u: U) -> u32 { unsafe { u.a } }
