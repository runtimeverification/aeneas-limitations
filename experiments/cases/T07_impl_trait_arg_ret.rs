pub fn evens(n: u32) -> impl Fn(u32) -> bool { move |x| x % 2 == 0 && x < n }
pub fn apply(f: impl Fn(u32) -> u32, x: u32) -> u32 { f(x) }
