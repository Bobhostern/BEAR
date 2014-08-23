use memory::Memory;
use std::io;
use unicode::char;

#[allow(dead_assignment)]
pub fn interpret(str: String, mem: &mut Memory) {
	let code_arr = str.into_bytes();
	let mut code_ptr = 0u;
	let mut reader = io::stdin();

	while code_ptr < code_arr.len() {
		let mut ainc = true;
		match code_arr[code_ptr] as char {
			'>' => mem.fwd(),
			'+' => mem.inc(),
			'-' => mem.dcr(),
			'<' => mem.bck(),
			'!' => print!("{}", match char::from_u32(mem.get() as u32) {
				Some(chr) => chr,
				None => fail!("Invalid unicode value: {}", mem.get())
			}),
			'*' => mem.lft(10),
			'/' => mem.dwn(10),
			')' => mem.adv(10),
			'(' => mem.rwd(10),
			'[' => {
				if mem.get() == 0 {
					while code_ptr < code_arr.len() && code_arr[code_ptr] as char != ']' { 
						code_ptr += 1; 
					}
					ainc = false;
				}
			},
			']' => {
				if mem.get() != 0 {
					while code_ptr > 0 && code_arr[code_ptr] as char != '[' { 
						code_ptr -= 1;
					}
					ainc = false;
				}
			},
			'?' => print!("{}", mem.getptr()),
			'&' => print!("{}", match char::from_u32(mem.getptr() as u32) {
				Some(chr) => chr,
				None => fail!("Invalid unicode value: {}", mem.get())
			}),
			'.' => print!("{}", mem.get()),
			'`' => {
				let inchr = match reader.read_byte() {
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
			code_ptr += 1;
		}
		ainc = true;
		//println!("{}", code_arr[code_ptr] as char);
	}

	// Prints the last accessed cell's value
	// println!("{}", mem.get());
}

/*
#[allow(dead_assignment)]
pub fn dummy_interpret(str: String, mem: &mut Memory) {
	let code_arr = str.into_bytes();
	let mut code_ptr = 0u;
	let mut input = io::stdin();

	while code_ptr < code_arr.len() {
		let mut ainc = true;
		match code_arr[code_ptr] as char {
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
					while code_ptr < code_arr.len() && code_arr[code_ptr] as char != ']' { 
						code_ptr += 1; 
					}
					ainc = false;
				}
			},
			']' => {
				println!("RBL");
				if mem.get() != 0 {
					while code_ptr > 0 && code_arr[code_ptr] as char != '[' { 
						code_ptr -= 1;
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
			code_ptr += 1;
		}
		ainc = true;
		//println!("{}", code_arr[code_ptr] as char);
	}
}
*/