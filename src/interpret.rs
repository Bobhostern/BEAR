use memory::Memory;
use std::io;

#[allow(dead_assignment)]
pub fn interpret(str: String, mem: &mut Memory) {
	let test_arr = str.into_bytes();
	let mut test_ptr = 0u;
	let mut input = io::stdin();

	while test_ptr < test_arr.len() {
		let mut ainc = true;
		match test_arr[test_ptr] as char {
			'>' => mem.fwd(),
			'+' => mem.inc(),
			'-' => mem.dcr(),
			'<' => mem.bck(),
			'!' => print!("{}", mem.get() as u8 as char),
			'*' => mem.lft(10),
			'/' => mem.dwn(10),
			')' => mem.adv(10),
			'(' => mem.rwd(10),
			'[' => {
				if mem.get() == 0 {
					while test_ptr < test_arr.len() && test_arr[test_ptr] as char != ']' { 
						test_ptr += 1; 
					}
					ainc = false;
				}
			},
			']' => {
				if mem.get() != 0 {
					while test_ptr > 0 && test_arr[test_ptr] as char != '[' { 
						test_ptr -= 1;
					}
					ainc = false;
				}
			},
			'?' => print!("{}", mem.getptr()),
			'&' => print!("{}", mem.getptr() as u8 as char),
			'.' => print!("{}", mem.get()),
			'`' => {
				let inchr = match input.read_byte() {
					Ok(val) => val,
					Err(err) => {
						if err.kind != io::EndOfFile {
							fail!(err)
						}
						else {
							0 // EOF
						}
					}
				};

				mem.set(inchr as int);
			},
			'^' => mem.setptr(0),
			'#' => mem.set(0),
			_ => { /* We're a pretty ignorant parser... */ }
		}
		if ainc {
			test_ptr += 1;
		}
		ainc = true;
		//println!("{}", test_arr[test_ptr] as char);
	}

	// Prints the last accessed cell's value
	// println!("{}", mem.get());
}

/*
#[allow(dead_assignment)]
pub fn dummy_interpret(str: String, mem: &mut Memory) {
	let test_arr = str.into_bytes();
	let mut test_ptr = 0u;
	let mut input = io::stdin();

	while test_ptr < test_arr.len() {
		let mut ainc = true;
		match test_arr[test_ptr] as char {
			'>' => {println!("FWD"); mem.fwd()},
			'+' => {println!("INC"); mem.inc()},
			'-' => {println!("DCR"); mem.dcr()},
			'<' => {println!("BCK"); mem.bck()},
			'!' => println!("PCC*"),
			'*' => {println!("LFT"); mem.lft(10)},
			'/' => {println!("DWN"); mem.dwn(10)},
			')' => {println!("ADV"); mem.adv(10)},
			'(' => {println!("RWD"); mem.rwd(10)},
			'[' => {
				println!("LBL");
				if mem.get() == 0 {
					while test_ptr < test_arr.len() && test_arr[test_ptr] as char != ']' { 
						test_ptr += 1; 
					}
					ainc = false;
				}
			},
			']' => {
				println!("RBL");
				if mem.get() != 0 {
					while test_ptr > 0 && test_arr[test_ptr] as char != '[' { 
						test_ptr -= 1;
					}
					ainc = false;
				}
			},
			'?' => println!("PPL*"),
			'&' => println!("PPC*"),
			'.' => println!("PCL*"),
			'`' => {
				println!("INP");
				let inchr = match input.read_byte() {
					Ok(val) => val,
					Err(err) => {
						if err.kind != io::EndOfFile {
							fail!(err)
						}
						else {
							0 // EOF
						}
					}
				};

				mem.set(inchr as int);
			},
			'^' => {println!("RPT"); mem.setptr(0)},
			'#' => {println!("RCV"); mem.set(0)},
			_ => { }
		}
		if ainc {
			test_ptr += 1;
		}
		ainc = true;
		//println!("{}", test_arr[test_ptr] as char);
	}
}
*/