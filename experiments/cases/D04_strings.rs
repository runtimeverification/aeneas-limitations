pub fn s_len(s: &str) -> usize { s.len() }
pub fn mk() -> String { let mut s = String::new(); s.push_str("ab"); s }
pub fn eq(a: &str, b: &str) -> bool { a == b }
pub fn lit() -> &'static str { "hello" }
