pub fn b() -> u32 { let x = Box::new(5u32); *x + 1 }
pub fn bmut(x: &mut Box<u32>) { **x = 3; }
pub enum List { Cons(u32, Box<List>), Nil }
pub fn head(l: &List) -> u32 { match l { List::Cons(h, _) => *h, List::Nil => 0 } }
