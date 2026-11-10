pub fn f(s: &[u32]) -> u32 {
    let mut i = 0;
    let mut r = 0;
    while i < s.len() {
        if s[i] == 0 {
            return 0;
        }
        if s[i] == 1 {
            break;
        }
        r += s[i];
        i += 1;
    }
    r + 1
}
