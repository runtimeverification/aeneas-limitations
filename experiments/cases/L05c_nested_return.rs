pub fn find_pair(n: u32, t: u32) -> Option<(u32, u32)> {
    let mut i = 0;
    while i < n {
        let mut j = 0;
        while j < n {
            if i + j == t {
                return Some((i, j));
            }
            j += 1;
        }
        i += 1;
    }
    None
}
