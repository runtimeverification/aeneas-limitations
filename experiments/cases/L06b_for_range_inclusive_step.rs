pub fn sum_incl(n: u32) -> u32 {
    let mut s = 0;
    for i in 0..=n {
        s += i;
    }
    s
}
pub fn sum_step(n: usize) -> usize {
    let mut s = 0;
    for i in (0..n).step_by(2) {
        s += i;
    }
    s
}
