pub const MEMORY_SIZE: usize = 4096;

pub struct Memory {
	mem: [i32; MEMORY_SIZE],
	memptr: usize
}

impl Memory {
	pub fn new() -> Memory {
		Memory { mem: [0; MEMORY_SIZE], memptr: 0 }
	}

	pub fn getptr(&self) -> usize {
		self.memptr
	}

	pub fn setptr(&mut self, loc: usize) {
		if loc < MEMORY_SIZE {
			self.memptr = loc;
		}
	}

	pub fn set(&mut self, val: i32) {
		self.mem[self.memptr] = val
	}

	pub fn get(&self) -> i32 {
		self.mem[self.memptr]
	}

	pub fn fwd(&mut self) {
		self.memptr += 1;
		if self.memptr >= MEMORY_SIZE {
			self.memptr = 0;
		}
	}

	pub fn adv(&mut self, by: u32) {
		let mut cnt = 0u32;
		while cnt < by {
			self.fwd();
			cnt += 1;
		}
	}

	pub fn bck(&mut self) {
		if self.memptr == 0 {
			self.memptr = MEMORY_SIZE - 1;
		}
		else {
			self.memptr -= 1;
		}
	}

	pub fn rwd(&mut self, by: u32) {
		let mut cnt = 0u32;
		while cnt < by {
			self.bck();
			cnt += 1;
		}
	}

	pub fn inc(&mut self) {
		self.mem[self.memptr] = self.mem[self.memptr] + 1;
	}

	pub fn lft(&mut self, by: u32) {
		let mut cnt = 0u32;
		while cnt < by {
			self.inc();
			cnt += 1;
		}
	}

	pub fn dcr(&mut self) {
		self.mem[self.memptr] = self.mem[self.memptr] - 1;
	}

	pub fn dwn(&mut self, by: u32) {
		let mut cnt = 0u32;
		while cnt < by {
			self.dcr();
			cnt += 1;
		}
	}
}
