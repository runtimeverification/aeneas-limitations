pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}
// Loop that walks a list and returns a mutable borrow into it.
pub fn nth_mut<T>(mut l: &mut List<T>, mut i: u32) -> Option<&mut T> {
    while let List::Cons(x, tl) = l {
        if i == 0 {
            return Some(x);
        }
        i -= 1;
        l = tl;
    }
    None
}
// Loop that returns a shared borrow into a slice.
pub fn first_ge(s: &[u32], k: u32) -> Option<&u32> {
    let mut i = 0;
    while i < s.len() {
        if s[i] >= k {
            return Some(&s[i]);
        }
        i += 1;
    }
    None
}
