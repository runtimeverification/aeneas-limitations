pub fn add_k(s: &[u32], k: u32) -> Vec<u32> {
    s.iter().map(|x| x + k).collect()
}
pub fn any_gt(s: &[u32], k: u32) -> bool {
    s.iter().any(|x| *x > k)
}
