pub fn last_nonzero(n: usize, s: &[u32]) -> Option<usize> {
    for i in (0..n).rev() {
        if s[i] != 0 {
            return Some(i);
        }
    }
    None
}
