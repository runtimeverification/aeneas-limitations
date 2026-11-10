pub fn build() -> Vec<u32> { let mut v = Vec::new(); v.push(1); v.push(2); v[0] = 5; v }
pub fn get(v: &Vec<u32>, i: usize) -> u32 { v[i] }
pub fn pop(v: &mut Vec<u32>) -> Option<u32> { v.pop() }
pub fn mac() -> Vec<u32> { vec![1, 2, 3] }
pub fn zeros(n: usize) -> Vec<u8> { vec![0; n] }
pub fn ins(v: &mut Vec<u32>) { v.insert(0, 9); v.remove(1); }
pub fn ext(v: &mut Vec<u32>, w: &[u32]) { v.extend_from_slice(w) }
