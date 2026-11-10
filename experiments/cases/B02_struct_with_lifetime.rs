pub struct View<'a> { pub data: &'a mut [u32], pub pos: usize }
impl<'a> View<'a> {
    pub fn set(&mut self, x: u32) { self.data[self.pos] = x; }
}
pub fn use_view(v: &mut [u32]) { let mut w = View { data: v, pos: 0 }; w.set(7); }
