pub fn weighted(s: &[u32]) -> usize {
    let mut t = 0;
    for (i, x) in s.iter().enumerate() {
        t += i * (*x as usize);
    }
    t
}
