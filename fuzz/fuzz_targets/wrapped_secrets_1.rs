#![no_main]
extern crate arbitrary;
#[macro_use]
extern crate libfuzzer_sys;
extern crate rusty_secrets;

use rusty_secrets::wrapped_secrets;
use arbitrary::{RingBuffer, Unstructured};

fuzz_target!(|data: &[u8]| {
    // ---
    if let Ok(mut buffer) = RingBuffer::new(data, data.len()) {
        let mut kn = vec![0; 2];
        buffer.fill_buffer(&mut kn).unwrap();

        let k = kn[0];
        let n = kn[1];

        wrapped_secrets::split_secret(k, n, &data, false)
            .map_err(|err| err.into())
            .and_then(|ss| wrapped_secrets::recover_secret(&ss, false))
            .map(|_| ())
            .unwrap_or(())
    }
});
