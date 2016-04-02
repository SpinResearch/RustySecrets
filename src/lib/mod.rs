extern crate rustc_serialize as serialize;
extern crate rand;

use self::rand::{ Rng, OsRng };
pub use self::serialize::base64::{ self, FromBase64, ToBase64 };

mod gf256;
use self::gf256::Gf256;

use std::io;
pub use std::str;
use std::iter::repeat;

pub mod custom_error;
use self::custom_error::*;

pub fn generate_shares(k: u8, n: u8, secret: Vec<u8>) -> io::Result<Vec<Vec<u8>>> {
	let shares = try!(secret_share(&*secret, k, n));
	let config = base64::Config {
		pad: false,
		..base64::STANDARD
	};

	let mut result = Vec::with_capacity(n as usize);

	for (index, share) in shares.iter().enumerate() {
		let salad = share.to_base64(config);
		let string = format!("{}-{}-{}", k, index+1, salad).into_bytes();
		result.push(string);
	}

	Ok(result)
}

pub fn recover_secret(k: u8, shares: Vec<(u8,Vec<u8>)>) -> io::Result<Vec<u8>> {
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

	return Ok(secret) as io::Result<Vec<u8>>;
}

fn new_vec<T: Clone>(n: usize, x: T) -> Vec<T> {
	repeat(x).take(n).collect()
}

/// evaluates a polynomial at x=1, 2, 3, ... n (inclusive)
fn encode<W: Write>(src: &[u8], n: u8, w: &mut W) -> io::Result<()> {
	for raw_x in 1 .. ((n as u16) + 1) {
		let x = Gf256::from_byte(raw_x as u8);
		let mut fac = Gf256::one();
		let mut acc = Gf256::zero();
		for &coeff in src.iter() {
			acc = acc + fac * Gf256::from_byte(coeff);
			fac = fac * x;
		}
		try!(w.write(&[acc.to_byte()]));
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

fn secret_share(src: &[u8], k: u8, n: u8) -> io::Result<Vec<Vec<u8>>> {
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
