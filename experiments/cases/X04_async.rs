pub async fn f(x: u32) -> u32 { x + 1 }
pub async fn g() -> u32 { f(1).await }
