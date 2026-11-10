pub fn dot(a: &[u32], b: &[u32]) -> u32 {
    let mut t = 0;
    for (x, y) in a.iter().zip(b.iter()) {
        t += x * y;
    }
    t
}
