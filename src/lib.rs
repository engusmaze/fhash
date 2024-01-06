use core::hash::Hasher;
use std::hash::BuildHasher;

pub mod internal;

pub struct FHasher {
    state: u64,
}
impl Default for FHasher {
    #[inline(always)]
    fn default() -> Self {
        Self { state: 0 }
    }
}
impl Hasher for FHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        internal::finish(self.state)
    }
    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        internal::mix_with_bytes(&mut self.state, bytes);
    }

    #[inline(always)]
    fn write_u8(&mut self, i: u8) {
        self.state = internal::mix_u64x2(self.state, i as u64)
    }

    #[inline(always)]
    fn write_u16(&mut self, i: u16) {
        self.state = internal::mix_u64x2(self.state, i as u64)
    }

    #[inline(always)]
    fn write_u32(&mut self, i: u32) {
        self.state = internal::mix_u64x2(self.state, i as u64)
    }

    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        self.state = internal::mix_u64x2(self.state, i as u64)
    }

    #[inline(always)]
    fn write_u128(&mut self, i: u128) {
        self.state = internal::mix_u64x3(self.state, i as u64, (i >> 64) as u64)
    }

    #[inline(always)]
    fn write_usize(&mut self, i: usize) {
        self.state = internal::mix_u64x2(self.state, i as u64)
    }
}

pub struct DefaultFHasherBuilder;
impl BuildHasher for DefaultFHasherBuilder {
    type Hasher = FHasher;
    #[inline(always)]
    fn build_hasher(&self) -> FHasher {
        FHasher::default()
    }
}
