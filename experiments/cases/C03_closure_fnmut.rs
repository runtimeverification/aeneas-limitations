pub fn call_twice<F: FnMut()>(mut f: F) { f(); f(); }
pub fn count() -> u32 { let mut c = 0; call_twice(|| c += 1); c }
