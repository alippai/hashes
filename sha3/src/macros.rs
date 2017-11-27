macro_rules! impl_state {
    ($state:ident, $rate:ident, $buffer:ty, $padding:ty) => {

        #[allow(non_camel_case_types)]
        #[derive(Clone, Default)]
        pub struct $state {
            state: Sha3State,
            buffer: $buffer,
        }

        impl $state {
            fn absorb(&mut self, input: &[u8]) {
                let self_state = &mut self.state;
                self.buffer.input(input, |b| self_state.absorb_block(b));
            }

            fn apply_padding(&mut self) {
                let buf = self.buffer.pad_with::<$padding>();
                self.state.absorb_block(buf);
            }
        }
    }
}

macro_rules! sha3_impl {
    (
        $state:ident, $output_size:ident, $rate:ident,
        $buffer:ty, $padding:ty
    ) => {

        impl_state!($state, $rate, $buffer, $padding);

        impl BlockInput for $state {
            type BlockSize = $rate;
        }

        impl Input for $state {
            fn process(&mut self, data: &[u8]) {
                self.absorb(data)
            }
        }

        impl FixedOutput for $state {
            type OutputSize = $output_size;

            fn fixed_result(&mut self) -> GenericArray<u8, Self::OutputSize> {
                self.apply_padding();

                let mut out = GenericArray::default();
                let n = out.len();
                self.state.as_bytes(|state| {
                    out.copy_from_slice(&state[..n]);
                });
                *self = Default::default();
                out
            }
        }

        impl_opaque_debug!($state);
        impl_write!($state);
    }
}

macro_rules! shake_impl {
    ($state:ident, $rate:ident, $buffer:ty, $padding:ty) => {
        impl_state!($state, $rate, $buffer, $padding);

        impl Input for $state {
            fn process(&mut self, data: &[u8]) {
                self.absorb(data)
            }
        }

        impl ExtendableOutput for $state {
            type Reader = Sha3XofReader;

            fn xof_result(&mut self) -> Sha3XofReader
            {
                self.apply_padding();
                let r = $rate::to_usize();
                let res = Sha3XofReader::new(self.state.clone(), r);
                *self = Default::default();
                res
            }
        }

        impl_opaque_debug!($state);
        impl_write!($state);
    }
}
