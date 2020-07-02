pub struct Context<'a> {
	input: &'a str,
	output: u64,
}

impl<'a> Context<'a> {
	pub fn new(input: &'a str) -> Context {
		Context { input, output: 0 }
	}

	pub fn get_romans(&self) -> &'a str {
		self.input
	}

	pub fn split(&mut self, index: usize) {
		let (_, last) = self.input.split_at(index);
		self.set_romans(last);
	}

	fn set_romans(&mut self, new_input: &'a str) {
		self.input = new_input;
	}

	pub fn add_number(&mut self, number: u64) {
		self.output += number;
	}

	pub fn print(&self) -> u64 {
		self.output
	}
}
