pub fn p(x: u32) -> u32 { if x == 0 { panic!("zero") } x }
pub fn a(x: u32) -> u32 { assert!(x > 1); x }
pub fn ae(x: u32) { assert_eq!(x, 1) }
pub fn u(x: Option<u32>) -> u32 { x.unwrap() }
pub fn e(x: Option<u32>) -> u32 { x.expect("none") }
pub fn un() -> u32 { unreachable!() }
