use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::fmt;
use std::num::Wrapping;

const POLY: u8 = 0x1D;

/// replicates the least significant bit to every other bit
#[inline]
fn mask(bit: u8) -> u8 {
    (Wrapping(0u8) - Wrapping(bit & 1)).0
}

/// multiplies a polynomial with x and returns the residual
/// of the polynomial division with POLY as divisor
#[inline]
fn xtimes(poly: u8) -> u8 {
	(poly << 1) ^ (mask(poly >> 7) & POLY)
}

struct Tables {
	exp: [u8; 256],
	log: [u8; 256],
}

fn generate_tables(mut file: &File) {
    let mut tabs = Tables {
    	exp: [0; 256],
    	log: [0; 256],
    };

    let mut tmp = 1;
    for power in 0..255usize {
    	tabs.exp[power] = tmp;
    	tabs.log[tmp as usize] = power as u8;
    	tmp = xtimes(tmp);
    }

    match write!(file, "{}", tabs) {
        Ok(()) => {}
        Err(_) => panic!("Could not format the table. Aborting build.")
    };
}

fn farray(array: [u8; 256], f: &mut fmt::Formatter) -> fmt::Result {
    for (index, value) in array.into_iter().enumerate() {
        try!(write!(f, "{}", value));
        if index != array.len()-1 {
            try!(write!(f, ","));
        }
    }
    Ok(())
}

impl fmt::Display for Tables {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Tables {{\n"));
        try!(write!(f, "    exp: ["));
        try!(farray(self.exp, f));
        try!(write!(f, "],\n"));
        try!(write!(f, "    log: ["));
        try!(farray(self.log, f));
        try!(write!(f, "]\n"));
        write!(f, "}};")
    }
}

#[allow(unused_must_use)]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("nothinghardcoded.rs");

    let mut f = File::create(&dest).unwrap();

    write!(f, "pub struct Tables {{
    pub exp: [u8; 256],
    pub log: [u8; 256]
}}

pub static TABLES: Tables = ");

    generate_tables(&f);
}
