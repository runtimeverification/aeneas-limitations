#[no_mangle]
pub extern "C" fn exported(x: u32) -> u32 { x.wrapping_add(1) }
