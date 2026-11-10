pub fn f(n: u32) -> u32 {
    let mut i = 0;
    let mut c = 0;
    'outer: while i < n {
        i += 1;
        let mut j = 0;
        while j < i {
            j += 1;
            if j == 3 {
                continue 'outer;
            }
            c += 1;
        }
    }
    c
}
