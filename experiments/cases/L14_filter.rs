pub fn count_even(s: &[u32]) -> usize {
    s.iter().filter(|x| **x % 2 == 0).count()
}
