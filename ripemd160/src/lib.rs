//! An implementation of the RIPEMD-160 cryptographic hash.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate byte_tools;
extern crate block_buffer;
#[macro_use] extern crate opaque_debug;
#[macro_use] extern crate digest;

pub use digest::Digest;
use digest::{Input, BlockInput, FixedOutput};
use byte_tools::write_u32v_le;
use block_buffer::BlockBuffer512;
use digest::generic_array::GenericArray;
use digest::generic_array::typenum::{U20, U64};

mod block;
use block::{process_msg_block, DIGEST_BUF_LEN, H0};

/// Structure representing the state of a Ripemd160 computation
#[derive(Clone)]
pub struct Ripemd160 {
    h: [u32; DIGEST_BUF_LEN],
    len: u64,
    buffer: BlockBuffer512,
}

impl Default for Ripemd160 {
    fn default() -> Self {
        Ripemd160 {
            h: H0,
            len: 0,
            buffer: Default::default(),
        }
    }
}

impl BlockInput for Ripemd160 {
    type BlockSize = U64;
}

impl Input for Ripemd160 {
    fn process(&mut self, input: &[u8]) {
        // Assumes that input.len() can be converted to u64 without overflow
        self.len += input.len() as u64;
        let h = &mut self.h;
        self.buffer.input(input, |b| process_msg_block(h, b));
    }
}

impl FixedOutput for Ripemd160 {
    type OutputSize = U20;

    fn fixed_result(&mut self) -> GenericArray<u8, Self::OutputSize> {
        {
            let h = &mut self.h;
            let l = self.len << 3;
            self.buffer.len_padding(l, |b| process_msg_block(h, b));
        }

        let mut out = GenericArray::default();
        write_u32v_le(&mut out[..], &self.h);
        *self = Default::default();
        out
    }
}

impl_opaque_debug!(Ripemd160);
impl_write!(Ripemd160);
