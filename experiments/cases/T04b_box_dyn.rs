pub trait Shape { fn area(&self) -> u32; }
pub struct Sq { pub s: u32 }
impl Shape for Sq { fn area(&self) -> u32 { self.s * self.s } }
pub fn mk() -> Box<dyn Shape> { Box::new(Sq { s: 3 }) }
pub fn use_box(b: Box<dyn Shape>) -> u32 { b.area() }
