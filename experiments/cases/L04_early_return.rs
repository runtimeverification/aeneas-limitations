pub fn find(s: &[u32], x: u32) -> Option<usize> {
    let mut i = 0;
    while i < s.len() {
        if s[i] == x {
            return Some(i);
        }
        i += 1;
    }
    None
}
