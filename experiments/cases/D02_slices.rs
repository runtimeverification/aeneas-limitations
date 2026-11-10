pub fn first(s: &[u32]) -> u32 { s[0] }
pub fn sub(s: &[u32]) -> &[u32] { &s[1..3] }
pub fn len(s: &[u32]) -> usize { s.len() }
pub fn split(s: &mut [u32]) -> u32 { let (a, b) = s.split_at_mut(1); a[0] + b[0] }
pub fn swap(s: &mut [u32]) { s.swap(0, 1) }
