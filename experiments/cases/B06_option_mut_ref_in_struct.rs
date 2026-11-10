pub struct Node { pub v: u32, pub next: Option<Box<Node>> }
pub fn last_mut(n: &mut Node) -> &mut u32 {
    match &mut n.next { None => &mut n.v, Some(nx) => last_mut(nx) }
}
