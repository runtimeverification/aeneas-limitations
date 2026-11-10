use std::rc::Rc;
pub fn rc(x: u32) -> u32 { let a = Rc::new(x); let b = Rc::clone(&a); *a + *b }
