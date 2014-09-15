use memory::Memory;
use memory;
use std::io;

#[allow(dead_assignment)]
pub fn interpret(str: String, mem: &mut Memory, debug: bool) {
	let mut code_arr = str.into_bytes();
	let mut code_ptr = 0u;
	let mut reader = io::stdin();

	while code_ptr < code_arr.len() {
		let mut ainc = true;
		match code_arr[code_ptr] as char {
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
			'&' => print!("{}", mem.getptr() as u8 as char),
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
			'$' => {
				if debug {
					// How far do we want to search?
					static SEARCH_RADIUS: uint = 5;
					// Save our current location
					let ptr = mem.getptr();
					// The list of indices to show
					let mut cellvec = Vec::new();
					// The number used to calculate the indices
					let mut vptr: int = ptr as int;

					// Do a -SEARCH_RADIUS amount of indices behind (with wrapping)
					vptr -= SEARCH_RADIUS as int;
					let mut cnt = 0u;
			 		while cnt < SEARCH_RADIUS {
						if vptr < 0 {
							cellvec.push(memory::MEMORY_SIZE - (-vptr) as uint);
						} else {
							cellvec.push(vptr as uint);
						}
						vptr += 1;
						cnt += 1;
					}

					// Put the current pointer location
					vptr = ptr as int;
					cnt = 0;
					cellvec.push(ptr);

					// Do a SEARCH_RADIUS amount of indices ahead (with wrapping)
					while cnt < SEARCH_RADIUS {
						vptr += 1;
						// Add then check for bounds, to properly limit the indice
						if (vptr as uint) == memory::MEMORY_SIZE {
							vptr = 0;
						}
						cellvec.push(vptr as uint);
						cnt += 1;
					}

					// Print the indicies
					for cellptr in cellvec.iter() {
						mem.setptr(cellptr.clone());
						println!("Cell #{}: {} ({})", cellptr, mem.get(), mem.get() as u8 as char);
					}
					// Reset the pointer
					mem.setptr(ptr);
				}
			},
			':' => {
				// Get proc key
				code_ptr += 1;
				let proc_key = code_arr[code_ptr];
				// Eat it =
				code_ptr += 1;
				let mut arr = Vec::new();
				while code_ptr < code_arr.len() && code_arr[code_ptr] as char != ';' { 
					arr = arr.append([code_arr[code_ptr]]);
					code_ptr += 1;
				}
				mem.add_proc(proc_key as char, match String::from_utf8(arr){
					Ok(val) => val,
					Err(err) => fail!(err)
				})
			},
			'\\' => {
				// Get proc key
				code_ptr += 1;
				let proc_key = code_arr[code_ptr];
				// Eat that char
				code_ptr += 1;
				let strg = mem.get_proc(proc_key as char).clone();
				//println!("{}", strg);

				code_arr = strg.into_bytes().append(code_arr.slice(code_ptr, code_arr.len()));
				// Reset the program pointer to the beginning
				code_ptr = 0;
				// Don't go forward: We just reset the pointer!
				ainc = false;
				/*println!("{}", match String::from_utf8(code_arr.clone()) {
					Ok(val) => val,
					Err(err) => fail!(err)
				})*/
			},
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