pub fn incr_all_idx(s: &mut [u32]) {
    let mut i = 0;
    while i < s.len() {
        s[i] = s[i].wrapping_add(1);
        i += 1;
    }
}
