use std::sync::atomic::{AtomicU32, Ordering};
pub fn a(x: &AtomicU32) -> u32 { x.fetch_add(1, Ordering::SeqCst) }
