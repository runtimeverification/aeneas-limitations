pub fn count_pairs(n: u32) -> u32 {
    let mut c = 0;
    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < i {
            c += 1;
            j += 1;
        }
        i += 1;
    }
    c
}
