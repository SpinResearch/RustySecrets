#![feature(collections)]
#![feature(core)]
#![feature(io)]
#![feature(env)]
#![feature(hash)]

extern crate "rustc-serialize" as serialize;
extern crate getopts;
extern crate crc24;
extern crate rand;

use std::iter::repeat;
use std::old_io::{ stdio, IoError, IoErrorKind, IoResult, BufferedReader };
use std::env;
use std::num;

use rand::{ Rng, OsRng };
use getopts::Options;
use serialize::base64::{ self, FromBase64, ToBase64 };

use gf256::Gf256;

mod gf256;

fn new_vec<T: Clone>(n: usize, x: T) -> Vec<T> {
	repeat(x).take(n).collect()
}

fn other_io_err(s: &'static str) -> IoError {
	IoError {
		kind: IoErrorKind::OtherIoError,
		desc: s,
		detail: None
	}
}

// a try!-like macro for Option<T> expressions that takes
// a &'static str as error message as 2nd parameter
// and creates an IoError out of it if necessary.
macro_rules! otry {
	($o:expr, $e:expr) => (
		match $o {
			Some(thing_) => thing_,
			None => return Err(other_io_err($e))
		}
	)
}

/// maps a ParseIntError to an IoError
fn pie2io(p: num::ParseIntError) -> IoError {
	IoError {
		detail: Some(p.to_string()),
		.. other_io_err("Int parsing error")
	}
}

/// evaluates a polynomial at x=1, 2, 3, ... n (inclusive)
fn encode<W: Writer>(src: &[u8], n: u8, w: &mut W) -> IoResult<()> {
	for raw_x in 1 .. ((n as u16) + 1) {
		let x = Gf256::from_byte(raw_x as u8);
		let mut fac = Gf256::one();
		let mut acc = Gf256::zero();
		for &coeff in src.iter() {
			acc = acc + fac * Gf256::from_byte(coeff);
			fac = fac * x;
		}
		try!(w.write_u8(acc.to_byte()));
	}
	Ok(())
}

/// evaluates an interpolated polynomial at `raw_x` where
/// the polynomial is determined using Lagrangian interpolation
/// based on the given x/y coordinates `src`.
fn lagrange_interpolate(src: &[(u8, u8)], raw_x: u8) -> u8 {
	let x = Gf256::from_byte(raw_x);
	let mut sum = Gf256::zero();
	for (i, &(raw_xi, raw_yi)) in src.iter().enumerate() {
		let xi = Gf256::from_byte(raw_xi);
		let yi = Gf256::from_byte(raw_yi);
		let mut lix = Gf256::one();
		for (j, &(raw_xj, _)) in src.iter().enumerate() {
			if i != j {
				let xj = Gf256::from_byte(raw_xj);
				let delta = xi - xj;
				assert!(delta.poly !=0, "Duplicate shares");
				lix = lix * (x - xj) / delta;
			}
		}
		sum = sum + lix * yi;
	}
	sum.to_byte()
}

fn secret_share(src: &[u8], k: u8, n: u8) -> IoResult<Vec<Vec<u8>>> {
	let mut result = Vec::with_capacity(n as usize);
	for _ in 0 .. (n as usize) {
		result.push(new_vec(src.len(), 0u8));
	}
	let mut col_in = new_vec(k as usize, 0u8);
	let mut col_out = Vec::with_capacity(n as usize);
	let mut osrng = try!(OsRng::new());
	for (c, &s) in src.iter().enumerate() {
		col_in[0] = s;
		osrng.fill_bytes(&mut col_in[1..]);
		col_out.clear();
		try!(encode(&*col_in, n, &mut col_out));
		for (&y, share) in col_out.iter().zip(result.iter_mut()) {
			share[c] = y;
		}
	}
	Ok(result)
}

enum Action {
	Encode(u8, u8), // k and n parameter
	Decode
}

fn parse_k_n(s: &str) -> IoResult<(u8, u8)> {
	let mut iter = s.split(',');
	let msg = "K and N have to be separated with a comma";
	let s1 = otry!(iter.next(), msg).trim();
	let s2 = otry!(iter.next(), msg).trim();
	let k = try!(s1.parse().map_err(pie2io));
	let n = try!(s2.parse().map_err(pie2io));
	Ok((k, n))
}

/// tries to read everything but stops early if the input seems to be
/// larger than `max` bytes and returns an IoError in this case
fn read_no_more_than<R: Reader>(r: &mut R, max: usize) -> IoResult<Vec<u8>> {
	let mut data = Vec::new();
	loop {
		if let Err(e) = r.push(max + 1 - data.len(), &mut data) {
			if e.kind == IoErrorKind::EndOfFile {
				break; // EOF condition is actually OK
			} else {
				return Err(e);
			}
		}
		if data.len() > max {
			return Err(other_io_err("Input too long"));
		}
	}
	Ok(data)
}

/// computes a CRC-24 hash over the concatenated coding parameters k, n
/// and the raw share data
fn crc24_as_bytes(k: u8, n: u8, octets: &[u8]) -> [u8; 3] {
	use std::hash::{ Hasher, Writer };
	use std::slice::ref_slice;

	let mut h = crc24::Crc24Hasher::new();
	h.write(ref_slice(&k));
	h.write(ref_slice(&n));
	h.write(octets);
	let v = h.finish();

	[((v >> 16) & 0xFF) as u8,
	 ((v >>  8) & 0xFF) as u8,
	 ( v        & 0xFF) as u8]
}

fn perform_encode(k: u8, n: u8, with_checksums: bool) -> IoResult<()> {
	let secret = try!(read_no_more_than(&mut stdio::stdin(), 0x10000));
	let shares = try!(secret_share(&*secret, k, n));
	let config = base64::Config {
		pad: false,
		..base64::STANDARD
	};
	for (index, share) in shares.iter().enumerate() {
		let salad = share.to_base64(config);
		if with_checksums {
			let crc_bytes = crc24_as_bytes(k, (index+1) as u8, &**share);
			println!("{}-{}-{}-{}", k, index+1, salad, crc_bytes.to_base64(config));
		} else {
			println!("{}-{}-{}", k, index+1, salad);
		}
	}
	Ok(())
}

/// reads shares from stdin and returns Ok(k, shares) on success
/// where shares is a Vec<(u8, Vec<u8>)> representing x-coordinates
/// and share data.
fn read_shares() -> IoResult<(u8, Vec<(u8,Vec<u8>)>)> {
	let mut stdin = BufferedReader::new(stdio::stdin());
	let mut opt_k_l: Option<(u8, usize)> = None;
	let mut counter = 0u8;
	let mut shares: Vec<(u8,Vec<u8>)> = Vec::new();
	for line in stdin.lines() {
		let line = try!(line);
		let parts: Vec<_> = line.trim().split('-').collect();
		if parts.len() < 3 || parts.len() > 4 {
			return Err(other_io_err("Share parse error: Expected 3 or 4 \
			                         parts searated by a minus sign"));
		}
		let (k, n, p3, opt_p4) = {
			let mut iter = parts.into_iter();
			let k = try!(iter.next().unwrap().parse::<u8>().map_err(pie2io));
			let n = try!(iter.next().unwrap().parse::<u8>().map_err(pie2io));
			let p3 = iter.next().unwrap();
			let opt_p4 = iter.next();
			(k, n, p3, opt_p4)
		};
		if k < 1 || n < 1 {
			return Err(other_io_err("Share parse error: Illegal K,N parameters"));
		}
		let data = try!(
			p3.from_base64().map_err(|_| other_io_err(
				"Share parse error: Base64 decoding of data block failed"))
		);
		if let Some(check) = opt_p4 {
			if check.len() != 4 {
				return Err(other_io_err("Share parse error: Checksum part is \
				                         expected to be four characters"));
			}
			let crc_bytes = try!(
				check.from_base64().map_err(|_| other_io_err(
					"Share parse error: Base64 decoding of checksum failed"))
			);
			let mychksum = crc24_as_bytes(k, n, &*data);
			if mychksum != crc_bytes {
				return Err(other_io_err("Share parse error: Checksum mismatch"));
			}
		}
		if let Some((ck, cl)) = opt_k_l {
			if ck != k || cl != data.len() {
				return Err(other_io_err("Incompatible shares"));
			}
		} else {
			opt_k_l = Some((k, data.len()));
		}
		if shares.iter().all(|s| s.0 != n) {
			shares.push((n, data));
			counter += 1;
			if counter == k {
				return Ok((k, shares));
			}
		}
	}
	Err(other_io_err("Not enough shares provided!"))
}

fn perform_decode() -> IoResult<()> {
	let (k, shares) = try!(read_shares());
	assert!(!shares.is_empty());
	let slen = shares[0].1.len();
	let mut col_in = Vec::with_capacity(k as usize);
	let mut secret = Vec::with_capacity(slen);
	for byteindex in 0 .. slen {
		col_in.clear();
		for s in shares.iter().take(k as usize) {
			col_in.push((s.0, s.1[byteindex]));
		}
		secret.push(lagrange_interpolate(&*col_in, 0u8));
	}
	let mut out = stdio::stdout_raw();
	try!(out.write_all(&*secret));
	out.flush()
}

fn main() {
	let mut stderr = stdio::stderr();
	let args: Vec<String> = env::args().collect();

	let mut opts = Options::new();
	opts.optflag("h", "help", "print this help text");
	opts.optflag("d", "decode", "for decoding");
	opts.optopt("e", "encode", "for encoding, K is the required number of \
	                            shares for decoding, N is the number of shares \
	                            to generate. 1 <= K <= N <= 255", "K,N");
	let opt_matches = match opts.parse(args.tail()) {
		Ok(m) => m,
		Err(f) => {
			drop(writeln!(&mut stderr, "Error: {}", f));
			env::set_exit_status(1);
			return;
		}
	};

	if args.len() < 2 || opt_matches.opt_present("h") {
		println!(
"The program secretshare is an implementation of Shamir's secret sharing scheme.\n\
 It is applied byte-wise within a finite field for arbitrarily long secrets.\n");
		println!("{}", opts.usage("Usage: secretshare [options]"));
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
			Ok(Action::Encode(k,n)) => perform_encode(k, n, true),
			Ok(Action::Decode) => perform_decode(),
			Err(e) => Err(other_io_err(e))
		};

	if let Err(e) = result {
		drop(writeln!(&mut stderr, "{}", e));
		env::set_exit_status(1);
	}
}

