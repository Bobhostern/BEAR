use memory::Memory;
use memory;
use std::io;

pub fn interpret(strg: String, mem: &mut Memory, debug: bool) {
	use std::io::Read;

	let code_arr = strg.into_bytes();
	let mut code_ptr = 0usize;
	let mut ainc = true;

	while code_ptr < code_arr.len() {
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
				loop {
					match io::stdin().bytes().take(1).next() {
						Some(Ok(inchr)) => {
							match inchr as i32 {
								4 | 26 => mem.set(0), // The cheat. RN, cannot detect EOF, so we do this...
							 	inchr => mem.set(inchr)
							}
						},
						Some(Err(err)) => {
							match err.kind() {
								io::ErrorKind::UnexpectedEof => mem.set(0),
								_ => panic!("{}", err)
							}
						},
						None => mem.set(0)
					}
					break
				}
			},
			'^' => mem.setptr(0),
			'#' => mem.set(0),
			'$' => {
				if debug {
					// How far do we want to search?
					static SEARCH_RADIUS: u32 = 5;
					// Save our current location
					let ptr = mem.getptr();
					// The list of indices to show
					let mut cellvec = Vec::new();
					// The number used to calculate the indices
					let mut vptr: i32 = ptr as i32;

					// Do a -SEARCH_RADIUS amount of indices behind (with wrapping)
					vptr -= SEARCH_RADIUS as i32;
					let mut cnt = 0u32;
			 		while cnt < SEARCH_RADIUS {
						if vptr < 0 {
							cellvec.push(memory::MEMORY_SIZE - (-vptr) as usize);
						} else {
							cellvec.push(vptr as usize);
						}
						vptr += 1;
						cnt += 1;
					}

					// Put the current pointer location
					vptr = ptr as i32;
					cnt = 0;
					cellvec.push(ptr);

					// Do a SEARCH_RADIUS amount of indices ahead (with wrapping)
					while cnt < SEARCH_RADIUS {
						vptr += 1;
						// Add then check for bounds, to properly limit the indice
						if (vptr as usize) == memory::MEMORY_SIZE {
							vptr = 0;
						}
						cellvec.push(vptr as usize);
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
			}
			_ => { /* We're a pretty ignorant parser... */ }
		}
		if ainc {
			code_ptr += 1;
		}
		ainc = true;
	}
}
