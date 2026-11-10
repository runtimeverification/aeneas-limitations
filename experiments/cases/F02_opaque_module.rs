// This module is marked opaque via `--opaque crate::hidden` on the Charon command line.
pub mod hidden {
    pub fn secret(x: u32) -> u32 { x.wrapping_mul(31) }
}
pub fn use_hidden(x: u32) -> u32 { hidden::secret(x) }
