use std::sync::Mutex;
pub fn m(x: &Mutex<u32>) { *x.lock().unwrap() += 1 }
