pub fn sum_slice(s: &[u32]) -> u32 {
    let mut t = 0;
    for x in s {
        t += *x;
    }
    t
}
