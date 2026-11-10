pub fn fact(n: u64) -> u64 { if n == 0 { 1 } else { n * fact(n - 1) } }
pub enum L { Cons(u32, Box<L>), Nil }
pub fn len(l: &L) -> u32 { match l { L::Cons(_, t) => 1 + len(t), L::Nil => 0 } }
