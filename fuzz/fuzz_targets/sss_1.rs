#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate rusty_secrets;
extern crate arbitrary;

use rusty_secrets::sss::SSS;
use arbitrary::{RingBuffer, Unstructured};

fuzz_target!(|data: &[u8]| {
    // ...
    if let Ok(mut buffer) = RingBuffer::new(data, data.len()) {
        let mut kn = vec![0; 2];
        buffer.fill_buffer(&mut kn).unwrap();

        let k = kn[0];
        let n = kn[1];

        let sss = SSS::default();
        sss.generate_shares(k, n, &data, false)
            .map_err(|err| err.into())
            .and_then(|ss| SSS::recover_secret(ss, false))
            .unwrap_or(Vec::new());
    }
});
