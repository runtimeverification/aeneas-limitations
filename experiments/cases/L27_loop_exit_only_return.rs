pub fn find_zero(s: &[u32]) -> usize {
    let mut i = 0;
    loop {
        if i >= s.len() || s[i] == 0 {
            return i;
        }
        i += 1;
    }
}
