// This code implements a custom hash function in Rust, using mixing and finalization operations.
// It includes helper functions for mixing two or three u64 integers and a function to hash byte sequences.

/// A prime number, the secret sauce for our hashing recipe!
pub const PRIME: u64 = 15422943418517532487;

/// Perform finalization on the hash
#[inline(always)]
pub fn finish(mut hash: u64) -> u64 {
    hash = (hash ^ hash >> 32).wrapping_mul(PRIME);
    hash ^ hash >> 32
}

/// Mix two u64 integers using addition and XOR
#[inline(always)]
pub fn mix2(a: u64, b: u64) -> u64 {
    a.wrapping_add(PRIME) ^ b
}
/// Mix three u64 integers using addition and XOR
#[inline(always)]
pub fn mix3(a: u64, b: u64, c: u64) -> u64 {
    a.wrapping_add(PRIME) ^ b.wrapping_add(c) ^ c
}

/// Mix hash with byte slice
#[inline(always)]
pub fn mix_with_bytes(hash: u64, bytes: &[u8]) -> u64 {
    unsafe {
        let mut hash = mix2(hash, bytes.len() as u64);
        let ptr = bytes.as_ptr();
        let end = ptr.wrapping_add(bytes.len());
        if bytes.len() > 16 {
            let mut u64_ptr = ptr.cast::<(u64, u64)>();
            let u64_end = end.cast::<(u64, u64)>().wrapping_sub(1);

            // Read the tail right away so we don't have to take care of it later on
            let (l, r) = u64_end.read_unaligned();
            hash = mix3(hash, l, r);

            // Read all the u64 pairs from start
            loop {
                let (l, r) = u64_end.read_unaligned();
                hash = mix3(hash, l, r);
                u64_ptr = u64_ptr.wrapping_add(1);
                if u64_ptr < u64_end {
                    continue;
                }
                break hash;
            }
        } else {
            // If the byte slice is smaller than or equal to 16 bytes, we have
            // to determine the appropriate integer size to read and hash

            /// Helper function to read different-sized integers from the byte slice
            unsafe fn read_lr<T: Into<u64>>(ptr: *const u8, end: *const u8) -> (u64, u64) {
                (
                    ptr.cast::<T>().read_unaligned().into(),
                    end.cast::<T>().wrapping_sub(1).read_unaligned().into(),
                )
            }

            // Determine the input hash based on the byte chunk length and mix3
            // the values with the use of the best if else practices
            let (l, r) = if bytes.len() == 0 {
                (PRIME, PRIME)
            } else if bytes.len() <= 2 {
                read_lr::<u8>(ptr, end)
            } else if bytes.len() <= 4 {
                read_lr::<u16>(ptr, end)
            } else if bytes.len() <= 8 {
                read_lr::<u32>(ptr, end)
            } else {
                read_lr::<u64>(ptr, end)
            };

            mix3(hash, l, r)
        }
    }
}
