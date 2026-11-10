pub fn first_pow2_ge(n: u32) -> u32 {
    let mut p = 1;
    loop {
        if p >= n {
            break;
        }
        p *= 2;
    }
    p
}
