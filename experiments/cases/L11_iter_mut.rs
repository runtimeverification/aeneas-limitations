pub fn incr_all(s: &mut [u32]) {
    for x in s.iter_mut() {
        *x = x.wrapping_add(1);
    }
}
