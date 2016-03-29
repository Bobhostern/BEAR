extern crate rustc_serialize;
extern crate docopt;

use std::fs::File;
use std::path::Path;
use memory::Memory;
use interpret::interpret;

use docopt::Docopt;
use std::io;

mod memory;
mod interpret;

#[derive(RustcDecodable)]
struct BearOpts {
    flag_file: Option<String>,
    flag_help: bool,
    flag_interactive: bool,
    flag_debug: bool
}

const USAGE: &'static str = "
BEAR - Another BF.

USAGE:
    bear (-f FILE | --file FILE | -i | --interactive) [-d | --debug]
    bear (-h | --help)

Options:
    -h, --help  Show usage description.
    -f FILE, --file FILE  Read file and interpret it.
    -d, --debug  Enter debug mode.
    -i, --interactive  Enter interactive mode.
";

fn interactive_console(opts: &BearOpts) {
    use std::io::BufRead;

    let mut mem = Memory::new();

    println!("BEAR Interactive Console ver. 1.0");
    print!("> ");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(val) => { interpret(val, &mut mem, opts.flag_debug); print!("\n> "); },
            Err(err) => panic!("{}", err)
        };
    }
}

fn main() {
    use std::io::Read;

    let opts: BearOpts = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    if opts.flag_help {
        println!("{}", USAGE);
    }
    else if opts.flag_interactive {
        interactive_console(&opts);
    }
    else {
        let filename = match opts.flag_file {
            Some(val) => val,
            None => panic!("--file required")
        };
        let test = match File::open(&Path::new(&filename)) {
            Ok(mut val) => {
                let mut s = String::new();
                val.read_to_string(&mut s).unwrap();
                s
            },
            Err(err) => panic!("{}", err)
        };

        let mut mem = Memory::new();

        interpret(test, &mut mem, opts.flag_debug);
    }
}
