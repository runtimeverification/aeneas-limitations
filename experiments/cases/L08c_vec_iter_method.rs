pub fn sum_vec_iter(v: &Vec<u32>) -> u32 {
    let mut t: u32 = 0;
    for x in v.iter() {
        t = t.wrapping_add(*x);
    }
    t
}
pub fn incr_vec(v: &mut Vec<u32>) {
    for x in v.iter_mut() {
        *x = x.wrapping_add(1);
    }
}
