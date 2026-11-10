pub fn spawn() -> u32 { std::thread::spawn(|| 1u32).join().unwrap() }
