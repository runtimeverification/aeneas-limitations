pub fn sum_slice_idx(s: &[u32]) -> u32 {
    let mut t = 0;
    let mut i = 0;
    while i < s.len() {
        t += s[i];
        i += 1;
    }
    t
}
pub fn sum_slice_for_idx(s: &[u32]) -> u32 {
    let mut t = 0;
    for i in 0..s.len() {
        t += s[i];
    }
    t
}
