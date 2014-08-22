#![feature(phase)]

extern crate serialize;
#[phase(plugin, link)]
extern crate hammer;

use std::io::File;
use memory::Memory;
use interpret::interpret;

use hammer::{decode_args, usage};
use std::os;

mod memory;
mod interpret;

#[deriving(Decodable, Show)]
struct BearOpts {
	file: String,
	help: bool
}

hammer_config!(BearOpts "BEAR - Another BF",
	|c| { 
		c.short("file", 'f')
    }
)

fn main() {
	let opts: BearOpts = match decode_args(os::args().tail()) {
		Ok(val) => val,
		Err(err) => fail!(err.message)
	};
	if opts.help {
		let (desc, usage_text) = usage::<BearOpts>(true);
	    println!("Usage: {}", os::args().get(0));
	    println!("{}", usage_text);
	    println!("{}", desc.unwrap());
	}
	else {
		let test = match File::open(&Path::new(opts.file)).read_to_end() {
			Ok(val) => { 
				match String::from_utf8(val) {
					Ok(val) => val,
					Err(err) => fail!(err)
				} 
			},
			Err(err) => fail!(err)
		};

		let mut mem = Memory::new();

		interpret(test, &mut mem);
	}
}