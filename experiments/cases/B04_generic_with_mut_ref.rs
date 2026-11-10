pub fn id<T>(x: T) -> T { x }
pub fn use_id() -> u32 { let mut x = 0; *id(&mut x) += 1; x }
