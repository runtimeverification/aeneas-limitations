pub struct M(pub u32);
impl From<u32> for M { fn from(x: u32) -> M { M(x) } }
pub fn conv(x: u32) -> M { x.into() }
pub fn b(x: u32) -> [u8; 4] { x.to_le_bytes() }
pub fn fb(b: [u8; 4]) -> u32 { u32::from_le_bytes(b) }
