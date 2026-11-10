pub trait Named { fn id(&self) -> u32; }
pub trait Greet: Named {
    fn greet(&self) -> u32 { self.id() + 1 }
}
pub struct P;
impl Named for P { fn id(&self) -> u32 { 41 } }
impl Greet for P {}
pub fn g<T: Greet>(x: &T) -> u32 { x.greet() }
