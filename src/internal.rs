const PRIME_A: u64 = 10489087524697319597;
const PRIME_B: u64 = 15723408691975468189;

#[inline(always)]
pub fn finish(mut value: u64) -> u64 {
    value = (value ^ value >> 32).wrapping_mul(PRIME_B);
    value ^ value >> 32
}

#[inline(always)]
pub fn mix_u64(a: u64) -> u64 {
    a.wrapping_mul(PRIME_A) ^ PRIME_B
}

#[inline(always)]
pub fn mix_u64x2(a: u64, b: u64) -> u64 {
    a.wrapping_mul(PRIME_A) ^ b.wrapping_mul(PRIME_B)
}

#[inline(always)]
pub fn mix_u64x3(a: u64, b: u64, c: u64) -> u64 {
    a.wrapping_mul(PRIME_A) ^ b.wrapping_mul(PRIME_A) ^ c.wrapping_mul(PRIME_B)
}

#[inline(always)]
pub fn mix_u64x3_b(a: u64, b: u64, c: u64) -> u64 {
    a.wrapping_mul(PRIME_A) ^ b.wrapping_mul(PRIME_B) ^ c.wrapping_mul(PRIME_B)
}

#[inline(always)]
// #[no_mangle]
pub fn mix_with_bytes(hash: &mut u64, bytes: &[u8]) {
    let bytes_ptr = bytes.as_ptr();
    let bytes_end = bytes.as_ptr().wrapping_add(bytes.len());

    macro_rules! read_end {
        ($ty: ty) => {{
            (
                bytes_ptr.cast::<$ty>().read_unaligned() as u64,
                bytes_end.cast::<$ty>().wrapping_sub(1).read_unaligned() as u64,
            )
        }};
    }

    unsafe {
        if bytes.len() > 16 {
            *hash = mix_u64x2(*hash, bytes.len() as u64);
            let mut u128_ptr = bytes_ptr.cast::<u128>();
            let u128_end = bytes_end.cast::<u128>().wrapping_sub(1);
            let value = u128_end.read_unaligned();
            *hash = mix_u64x3_b(*hash, value as u64, (value >> 64) as u64);
            loop {
                let value = u128_ptr.read_unaligned();
                *hash = mix_u64x3(*hash, value as u64, (value >> 64) as u64);
                u128_ptr = u128_ptr.wrapping_add(1);
                if u128_ptr < u128_end {
                    continue;
                }
                break;
            }
        } else {
            *hash = mix_u64x3(
                *hash,
                if bytes.len() <= 8 {
                    if bytes.len() == 0 {
                        PRIME_A
                    } else {
                        let (l, r) = if bytes.len() <= 2 {
                            read_end!(u8)
                        } else {
                            if bytes.len() <= 4 {
                                read_end!(u16)
                            } else {
                                read_end!(u32)
                            }
                        };
                        l | r << 32
                    }
                } else {
                    let (l, r) = read_end!(u64);
                    mix_u64x2(l, r)
                },
                bytes.len() as u64,
            );
        }
    }
}
