#[derive(Clone, Copy, PartialEq)]
pub enum Color { R, G, B(u8) }
pub struct P { pub x: i32, pub y: i32 }
pub fn f(c: Color) -> u8 { match c { Color::R => 0, Color::G => 1, Color::B(v) if v > 10 => v, Color::B(_) => 2 } }
pub fn mv(p: &mut P) { p.x += 1; }
pub fn eqc(a: Color, b: Color) -> bool { a == b }
