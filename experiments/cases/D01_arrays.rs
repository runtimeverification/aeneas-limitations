pub fn arr() -> u32 { let mut a = [1u32, 2, 3, 4]; a[2] = 10; a[0] + a[2] + (a.len() as u32) }
pub fn swap01(a: &mut [u32; 2]) { let t = a[0]; a[0] = a[1]; a[1] = t; }
pub fn arr2d(m: [[u8; 2]; 2]) -> u8 { m[1][0] }
