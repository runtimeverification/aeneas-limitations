pub trait Shape { fn area(&self) -> u32; }
pub struct Sq { pub s: u32 }
impl Shape for Sq { fn area(&self) -> u32 { self.s * self.s } }
pub fn area_dyn(x: &dyn Shape) -> u32 { x.area() }
pub fn call() -> u32 { let s = Sq { s: 3 }; area_dyn(&s) }
