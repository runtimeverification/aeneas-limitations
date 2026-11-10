#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct W(pub u32, pub u64);
pub fn c(w: &W) -> W { w.clone() }
pub fn d() -> W { W::default() }
pub fn e(a: &W, b: &W) -> bool { a == b }
