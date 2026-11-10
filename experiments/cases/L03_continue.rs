pub fn sum_even(n: u32) -> u32 {
    let mut i = 0;
    let mut s = 0;
    while i < n {
        i += 1;
        if i % 2 == 1 {
            continue;
        }
        s += i;
    }
    s
}
