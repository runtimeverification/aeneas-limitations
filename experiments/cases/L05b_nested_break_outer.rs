pub fn find_pair(n: u32, t: u32) -> u32 {
    let mut i = 0;
    let mut r = 0;
    'outer: while i < n {
        let mut j = 0;
        while j < n {
            if i + j == t {
                r = i;
                break 'outer;
            }
            j += 1;
        }
        i += 1;
    }
    r
}
