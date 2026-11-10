pub fn choose<'a>(b: bool, x: &'a mut u32, y: &'a mut u32) -> &'a mut u32 { if b { x } else { y } }
pub fn use_choose() -> (u32, u32) {
    let mut x = 0; let mut y = 0;
    { let r = choose(true, &mut x, &mut y); *r = 1; }
    (x, y)
}
pub fn get_mut_field(p: &mut (u32, u32)) -> &mut u32 { &mut p.1 }
