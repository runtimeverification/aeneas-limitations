pub fn apply<F: Fn(u32) -> u32>(f: F, x: u32) -> u32 { f(x) }
pub fn use_apply(k: u32) -> u32 { apply(|y| y + k, 3) }
