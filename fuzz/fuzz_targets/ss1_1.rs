#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate rusty_secrets;
extern crate arbitrary;

use rusty_secrets::dss::ss1::*;
use arbitrary::{RingBuffer, Unstructured};

fuzz_target!(|data: &[u8]| {
    // ---
    if let Ok(mut buffer) = RingBuffer::new(data, data.len()) {
        let mut kn = vec![0; 2];
        buffer.fill_buffer(&mut kn).unwrap();

        let k = kn[0];
        let n = kn[1];

        split_secret(k, n, &data, Reproducibility::reproducible(), &None)
            .and_then(|ss| recover_secret(&ss))
            .map(|_| ())
            .unwrap_or(())
    }
});
