pub fn sum_fold(s: &[u32]) -> u32 {
    s.iter().fold(0, |acc, x| acc + x)
}
