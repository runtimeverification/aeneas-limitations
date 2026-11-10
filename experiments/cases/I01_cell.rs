use std::cell::Cell;
pub fn c(x: &Cell<u32>) { x.set(x.get() + 1) }
