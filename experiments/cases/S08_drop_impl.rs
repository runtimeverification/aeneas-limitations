pub struct G { pub x: u32 }
impl Drop for G { fn drop(&mut self) {} }
pub fn d() -> u32 { let g = G { x: 1 }; g.x }
