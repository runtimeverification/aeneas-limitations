pub fn sum_vec_owned(v: Vec<u32>) -> u32 {
    let mut t = 0;
    for x in v {
        t += x;
    }
    t
}
