pub fn f(s: &[u32]) -> Option<usize> {
    let mut i = 0;
    while i < s.len() {
        if s[i] == 0 {
            return Some(i);
        }
        i += 1;
    }
    let mut j = 0;
    while j < s.len() {
        if s[j] == 1 {
            return Some(j);
        }
        j += 1;
    }
    None
}
