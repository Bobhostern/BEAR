use std::collections::HashMap;
pub static MEMORY_SIZE: uint = 4096;

pub struct Memory {
	mem: [int, ..MEMORY_SIZE],
	procs: HashMap<char, String>,
	memptr: uint
}

impl Memory {
	pub fn new() -> Memory {
		Memory { mem: [0, ..MEMORY_SIZE], procs: HashMap::new(), memptr: 0 }
	}

	pub fn getptr(&self) -> uint {
		self.memptr
	}

	pub fn setptr(&mut self, loc: uint) {
		if loc < MEMORY_SIZE {
			self.memptr = loc;
		}
	}

	pub fn set(&mut self, val: int) {
		self.mem[self.memptr] = val
	}

	pub fn get(&self) -> int {
		self.mem[self.memptr]
	}

	pub fn fwd(&mut self) {
		self.memptr += 1;
		if self.memptr >= MEMORY_SIZE {
			self.memptr = 0;
		}
	}

	pub fn adv(&mut self, by: uint) {
		let mut cnt = 0u;
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

	pub fn rwd(&mut self, by: uint) {
		let mut cnt = 0u;
		while cnt < by {
			self.bck();
			cnt += 1;
		}
	}

	pub fn inc(&mut self) {
		self.mem[self.memptr] += 1;
	}

	pub fn lft(&mut self, by: uint) {
		let mut cnt = 0u;
		while cnt < by {
			self.inc();
			cnt += 1;
		}
	}

	pub fn dcr(&mut self) {
		self.mem[self.memptr] -= 1;
	}

	pub fn dwn(&mut self, by: uint) {
		let mut cnt = 0u;
		while cnt < by {
			self.dcr();
			cnt += 1;
		}
	}

	pub fn add_proc(&mut self, chr: char, strng: String) {
		self.procs.insert(chr, strng);
	}

	pub fn get_proc(&self, chr: char) -> &String {
		match self.procs.find(&chr) {
			Some(val) => val,
			None => fail!("Proc not found: {}", chr)
		}
	}
}