pub fn zero_out(v: &mut Vec<u32>) {
    let mut i = 0;
    while i < v.len() {
        v[i] = 0;
        i += 1;
    }
}
pub struct Counter { pub n: u32 }
pub fn bump(c: &mut Counter, k: u32) {
    let mut i = 0;
    while i < k {
        c.n = c.n.wrapping_add(1);
        i += 1;
    }
}
