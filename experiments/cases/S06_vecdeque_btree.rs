use std::collections::{BTreeMap, VecDeque};
pub fn q() -> Option<u32> { let mut d = VecDeque::new(); d.push_back(1u32); d.pop_front() }
pub fn b() -> usize { let mut m = BTreeMap::new(); m.insert(1u32, 1u32); m.len() }
