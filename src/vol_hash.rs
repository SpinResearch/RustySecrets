use std;
use std::mem::transmute;

use ring::digest::{Algorithm, Context};

#[allow(unsafe_code)]
fn u32_to_bytes(x: u32) -> [u8; 4] {
    unsafe { transmute(x.to_be()) }
}

pub struct VOLHash {
    algorithm: &'static Algorithm,
    bytes: Vec<u8>,
}

impl VOLHash {
    pub fn new(algorithm: &'static Algorithm) -> VOLHash {
        Self {
            algorithm,
            bytes: Vec::new(),
        }
    }

    pub fn process(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes)
    }

    pub fn finish(self, dest: &mut [u8]) {
        let len = dest.len();
        assert!(len < std::u32::MAX as usize);

        let mut ctx = Context::new(self.algorithm);
        ctx.update(&[0u8]);
        ctx.update(&u32_to_bytes(len as u32));
        ctx.update(&self.bytes);

        let mut state = ctx.finish().as_ref().to_vec();

        let iter_num = len / self.algorithm.output_len;

        for i in 0..iter_num {
            let mut inner_ctx = Context::new(self.algorithm);
            inner_ctx.update(&[255u8]);
            inner_ctx.update(&u32_to_bytes(1 + i as u32));
            inner_ctx.update(&state);

            state.extend_from_slice(inner_ctx.finish().as_ref())
        }

        assert!(state.len() >= len);

        let src = state.drain(0..len).collect::<Vec<_>>();
        dest.copy_from_slice(&src);
    }
}
