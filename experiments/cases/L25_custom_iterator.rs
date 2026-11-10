pub struct Countdown { pub n: u32 }
impl Iterator for Countdown {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        if self.n == 0 { None } else { self.n -= 1; Some(self.n) }
    }
}
pub fn total(n: u32) -> u32 {
    let mut t: u32 = 0;
    for x in (Countdown { n }) {
        t = t.wrapping_add(x);
    }
    t
}
