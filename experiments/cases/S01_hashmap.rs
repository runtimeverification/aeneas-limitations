use std::collections::HashMap;
pub fn h() -> Option<u32> { let mut m = HashMap::new(); m.insert(1u32, 2u32); m.get(&1).copied() }
