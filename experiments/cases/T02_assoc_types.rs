pub trait Container { type Item; fn get(&self, i: usize) -> Option<&Self::Item>; }
pub struct Wrap { pub v: Vec<u32> }
impl Container for Wrap {
    type Item = u32;
    fn get(&self, i: usize) -> Option<&u32> { if i < self.v.len() { Some(&self.v[i]) } else { None } }
}
pub fn first<C: Container>(c: &C) -> Option<&C::Item> { c.get(0) }
