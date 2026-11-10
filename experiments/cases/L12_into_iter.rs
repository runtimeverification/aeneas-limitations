pub fn sum_into_iter(v: Vec<u32>) -> u32 {
    let mut t = 0;
    for x in v.into_iter() {
        t += x;
    }
    t
}
