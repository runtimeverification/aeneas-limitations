pub struct Buf<const N: usize> { pub data: [u8; N] }
pub fn len_of<const N: usize>(b: &Buf<N>) -> usize { N }
pub fn get0<const N: usize>(b: &Buf<N>) -> u8 { b.data[0] }
pub fn mk() -> Buf<4> { Buf { data: [0; 4] } }
