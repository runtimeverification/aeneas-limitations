pub fn sum_array(a: [u32; 4]) -> u32 {
    let mut t = 0;
    for x in a {
        t += x;
    }
    t
}
pub fn sum_array_ref(a: &[u32; 4]) -> u32 {
    let mut t = 0;
    for x in a {
        t += *x;
    }
    t
}
