use std::sync::Arc;
pub fn arc(x: u32) -> u32 { let a = Arc::new(x); let b = a.clone(); *b }
