use std::cell::RefCell;
pub fn r(x: &RefCell<u32>) { *x.borrow_mut() += 1 }
