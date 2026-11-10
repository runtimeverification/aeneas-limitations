pub fn div(a: u32, b: u32) -> Result<u32, ()> { if b == 0 { Err(()) } else { Ok(a / b) } }
pub fn chain(a: u32, b: u32, c: u32) -> Result<u32, ()> { let x = div(a, b)?; div(x, c) }
pub fn opt(x: Option<u32>) -> u32 { match x { Some(v) => v, None => 0 } }
pub fn unwrap_or(x: Option<u32>) -> u32 { x.unwrap_or(7) }
pub fn map_opt(x: Option<u32>) -> Option<u32> { x.map(|v| v + 1) }
pub fn q(x: Option<u32>) -> Option<u32> { let v = x?; Some(v) }
