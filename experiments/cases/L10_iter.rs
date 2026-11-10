pub fn count_gt(s: &[u32], k: u32) -> u32 {
    let mut c = 0;
    for x in s.iter() {
        if *x > k {
            c += 1;
        }
    }
    c
}
