pub struct Pair<A, B> { pub a: A, pub b: B }
impl<A: Copy, B: Copy> Pair<A, B> {
    pub fn swap(&self) -> Pair<B, A> { Pair { a: self.b, b: self.a } }
}
pub fn use_pair() -> u8 { Pair { a: 1u8, b: true }.swap().b }
