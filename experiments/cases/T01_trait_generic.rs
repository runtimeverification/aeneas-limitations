pub trait Shape { fn area(&self) -> u32; }
pub struct Sq { pub s: u32 }
impl Shape for Sq { fn area(&self) -> u32 { self.s * self.s } }
pub fn total_area<T: Shape>(a: &T, b: &T) -> u32 { a.area() + b.area() }
pub fn use_it() -> u32 { total_area(&Sq { s: 2 }, &Sq { s: 3 }) }
