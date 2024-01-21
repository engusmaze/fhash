#![cfg_attr(not(feature = "std"), no_std)]

use core::{
    hash::{BuildHasher, Hasher},
    mem,
};

pub mod internal;

/// Stupidly simple hasher.
pub struct FHasher {
    state: u64,
}
impl Default for FHasher {
    /// Creates a new FHasher with the default state (0).
    #[inline(always)]
    fn default() -> Self {
        Self { state: 0 }
    }
}
impl From<u64> for FHasher {
    /// Use custom u64 value as a seed for FHasher.
    #[inline(always)]
    fn from(state: u64) -> Self {
        Self { state }
    }
}
impl Hasher for FHasher {
    #[inline(always)]
    fn finish(&self) -> u64 {
        internal::finish(self.state)
    }

    #[inline(always)]
    fn write(&mut self, bytes: &[u8]) {
        self.state = internal::mix_with_bytes(self.state, bytes);
    }

    #[inline(always)]
    fn write_u8(&mut self, i: u8) {
        self.write_u64(i as u64);
    }

    #[inline(always)]
    fn write_u16(&mut self, i: u16) {
        self.write_u64(i as u64);
    }

    #[inline(always)]
    fn write_u32(&mut self, i: u32) {
        self.write_u64(i as u64);
    }

    #[inline(always)]
    fn write_u64(&mut self, i: u64) {
        self.state = internal::mix2(self.state, i as u64);
    }

    #[inline(always)]
    fn write_u128(&mut self, i: u128) {
        self.state = internal::mix3(self.state, i as u64, (i >> 64) as u64);
    }

    /// Updates the hasher with a single usize value.
    #[inline(always)]
    #[cfg(any(
        target_pointer_width = "16",
        target_pointer_width = "32",
        target_pointer_width = "64",
    ))]
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }

    /// Updates the hasher with a single usize value (for 128-bit systems).
    #[inline(always)]
    #[cfg(target_pointer_width = "128")]
    fn write_usize(&mut self, i: usize) {
        self.write_u128(i as u128);
    }
}

/// State used for the random hashing algorithm in `HashMap` and `HashSet`.
/// `RandomState` introduces randomness into the hashing process to mitigate certain types
/// of hash collision attacks, enhancing the security and resilience of the data structures.
/// It is designed to make it harder for attackers to predict hash values and exploit
/// vulnerabilities in the hash function.
///
/// # Example
///
/// ```
/// use hashbrown::HashMap;
/// use fhash::RandomState;
///
/// // Create a HashMap using RandomState as the hasher
/// let mut map: HashMap<i32, &str, RandomState> = HashMap::default();
///
/// // Inserting values into the HashMap
/// map.insert(1, "apple");
/// map.insert(2, "banana");
///
/// // Retrieve values from the HashMap
/// assert_eq!(map.get(&1), Some(&"apple"));
/// assert_eq!(map.get(&2), Some(&"banana"));
/// ```
pub struct RandomState {
    state: u64,
}
impl RandomState {
    #[inline(always)]
    pub const fn with_seed(seed: u64) -> Self {
        Self { state: seed }
    }
}
#[cfg(feature = "std")]
impl Default for RandomState {
    /// Creates a new RandomState with state seeded from current time.
    #[inline]
    fn default() -> Self {
        let (l, r) = unsafe { mem::transmute(std::time::Instant::now()) }; // Wild speed hack
        Self {
            state: internal::finish(internal::mix2(l, r)),
        }
    }
}
impl From<u64> for RandomState {
    /// Use custom u64 value as a seed for RandomState.
    /// Useful in `no_std` scenarios or environments with limited standard library access
    /// where direct construction from a u64 value is beneficial.
    #[inline(always)]
    fn from(state: u64) -> Self {
        Self { state }
    }
}
impl BuildHasher for RandomState {
    type Hasher = FHasher;

    /// Builds a new FHasher using the current RandomState's state.
    #[inline(always)]
    fn build_hasher(&self) -> FHasher {
        FHasher { state: self.state }
    }
}
