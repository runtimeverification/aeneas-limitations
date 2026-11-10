pub trait Zero { fn zero() -> Self; }
impl Zero for u32 { fn zero() -> u32 { 0 } }
impl Zero for u64 { fn zero() -> u64 { 0 } }
pub fn z<T: Zero>() -> T { T::zero() }
pub fn both() -> (u32, u64) { (z(), z()) }
