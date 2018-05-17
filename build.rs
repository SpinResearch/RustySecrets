extern crate protoc_rust;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::num::Wrapping;
use std::path::Path;

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
    tabs.exp[255] = 1;

    match write!(file, "{}", tabs) {
        Ok(()) => {}
        Err(_) => panic!("Could not format the table. Aborting build."),
    };
}

fn farray(array: [u8; 256], f: &mut fmt::Formatter) -> fmt::Result {
    for (index, value) in array.into_iter().enumerate() {
        try!(write!(f, "{}", value));
        if index != array.len() - 1 {
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

fn build_protobuf<'a>(
    out_dir: &'a str,
    input: &'a [&'a str],
    includes: &'a [&'a str],
) {
    use self::protoc_rust::{run, Args, Customize};
    run(
        Args {
            out_dir,
            input,
            includes,
            customize: Customize {
                ..Default::default()
            },
        }
    ).expect("protoc");
}

fn generate_gf256_table() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest = Path::new(&out_dir).join("nothinghardcoded.rs");

    let mut f = File::create(&dest).unwrap();

    write!(
        f,
        "pub struct Tables {{ \
         pub exp: [u8; 256], \
         pub log: [u8; 256] \
         }} \
         \
         pub static TABLES: Tables = "
    );

    generate_tables(&f);
}

#[allow(unused_must_use)]
fn main() {
    generate_gf256_table();
    build_protobuf("src/proto", &["protobuf/version.proto"], &[]);
    build_protobuf(
        "src/proto/dss",
        &[
            "protobuf/dss/metadata.proto",
            "protobuf/dss/secret.proto",
            "protobuf/dss/share.proto",
        ],
        &["protobuf", "protobuf/dss"]
    );
    build_protobuf(
        "src/proto/wrapped",
        &[
            "protobuf/wrapped/secret.proto",
            "protobuf/wrapped/share.proto"
        ],
        &["protobuf", "protobuf/dss"]
    );
}
