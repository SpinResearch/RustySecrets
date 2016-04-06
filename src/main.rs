extern crate getopts;

use getopts::Options;

use lib::custom_error::*;
mod lib;

use std::io;
use std::env;

enum Action {
	Encode(u8, u8), // k and n parameter
	Decode
}

// a try!-like macro for Option<T> expressions that takes
// a &'static str as error message as 2nd parameter
// and creates an Error out of it if necessary.
macro_rules! otry {
	($o:expr, $e:expr) => (
		match $o {
			Some(thing_) => thing_,
			None => return Err(convert::From::from(Error::new($e, None)))
		}
	)
}

fn main() {
	let mut stderr = io::stderr();
	let args: Vec<String> = env::args().collect();

	let mut opts = Options::new();
	opts.optflag("h", "help", "print this help text");
	opts.optflag("d", "decode", "for decoding");
	opts.optopt("e", "encode", "for encoding, K is the required number of \
	                            shares for decoding, N is the number of shares \
	                            to generate. 1 <= K <= N <= 255", "K,N");
	let opt_matches = match opts.parse(&args[1..]) {
		Ok(m) => m,
		Err(f) => {
			drop(writeln!(&mut stderr, "Error: {}", f));
			// env::set_exit_status(1); // FIXME: unstable feature
			return;
		}
	};

	if args.len() < 2 || opt_matches.opt_present("h") {
		println!(
"The program secretshare is an implementation of Shamir's secret sharing scheme.\n\
 It is applied byte-wise within a finite field for arbitrarily long secrets.\n");
		println!("{}", opts.usage("Usage: rustysecrets [options]"));
		println!("Input is read from STDIN and output is written to STDOUT.");
 		return;
	}

	let action: Result<_,_> =
		match (opt_matches.opt_present("e"), opt_matches.opt_present("d")) {
			(false, false) => Err("Nothing to do! Use -e or -d"),
			(true, true) => Err("Use either -e or -d and not both"),
			(false, true) => Ok(Action::Decode),
			(true, false) => {
				if let Some(param) = opt_matches.opt_str("e") {
					if let Ok((k,n)) = parse_k_n(&*param) {
						if 0 < k && k <= n {
							Ok(Action::Encode(k,n))
						} else {
							Err("Invalid encoding parameters K,N")
						}
					} else {
						Err("Could not parse K,N parameters")
					}
				} else {
					Err("No parameter for -e or -d provided")
				}
			}
		};

	let result =
		match action {
			Ok(Action::Encode(k,n)) => perform_encode_from_io(k, n),
			Ok(Action::Decode) => perform_decode_from_io(),
			Err(e) => Err(other_io_err(e, None))
		};

	if let Err(e) = result {
		drop(writeln!(&mut stderr, "{}", e));
		// env::set_exit_status(1); // FIXME: unstable feature
	}
}

fn perform_encode_from_io(k: u8, n: u8) -> io::Result<()> {
	let secret = {
        let limit: usize = 0x10000;
        let stdin = io::stdin();
        let mut locked = stdin.lock();
        let mut tmp: Vec<u8> = Vec::new();
        try!(locked.by_ref().take(limit as u64).read_to_end(&mut tmp));
        if tmp.len() == limit {
            let mut dummy = [0u8];
            if try!(locked.read(&mut dummy)) > 0 {
                return Err(other_io_err("Secret too large",
                                        Some(format!("My limit is at {} bytes.", limit))));
            }
        }
        tmp.pop();
		tmp
    };
	match lib::generate_shares(k, n, &secret) {
		Ok(shares) => {
			for share in shares {println!("{:?}", share)};
		}
		Err(e) => { return Err(e) as io::Result<()>; }
	}

	return Ok(()) as io::Result<()>;
}

fn perform_decode_from_io() -> io::Result<()> {
	let stdin = io::stdin();
	let stdin = io::BufReader::new(stdin.lock());
	let mut shares: Vec<String> = Vec::new();

	for line in stdin.lines() {
		match line {
			Ok(share) => shares.push(share),
			Err(_) => {}
		}
	}

	return match lib::recover_secret(shares) {
		Ok(secret) => {
			let mut out = io::stdout();
			try!(out.write_all(&*secret));
			out.flush()
		}
		Err(e) => {Err(e) as io::Result<()>}
	};
}

fn parse_k_n(s: &str) -> io::Result<(u8, u8)> {
	let mut iter = s.split(',');
	let msg = "K and N have to be separated with a comma";
	let s1 = otry!(iter.next(), msg).trim();
	let s2 = otry!(iter.next(), msg).trim();
	let k = try!(s1.parse().map_err(pie2io));
	let n = try!(s2.parse().map_err(pie2io));
	Ok((k, n))
}
