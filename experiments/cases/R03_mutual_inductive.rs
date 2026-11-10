pub enum Tree { Node(u32, Forest) }
pub enum Forest { Nil, Cons(Box<Tree>, Box<Forest>) }
pub fn tsum(t: &Tree) -> u32 { match t { Tree::Node(v, f) => v + fsum(f) } }
pub fn fsum(f: &Forest) -> u32 { match f { Forest::Nil => 0, Forest::Cons(t, r) => tsum(t) + fsum(r) } }
