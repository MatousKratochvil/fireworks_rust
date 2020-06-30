pub fn fibonacci(index: u64) -> u64 {
	match index {
		0 => 1,
		1 => 1,
		index => fibonacci(index-1) + fibonacci(index-2),
	}
}