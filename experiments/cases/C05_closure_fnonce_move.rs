pub fn consume<F: FnOnce() -> Vec<u32>>(f: F) -> Vec<u32> { f() }
pub fn mk(v: Vec<u32>) -> Vec<u32> { consume(move || v) }
