pub fn drain_sum(mut v: Vec<u32>) -> u32 {
    let mut t: u32 = 0;
    while let Some(x) = v.pop() {
        t = t.wrapping_add(x);
    }
    t
}
