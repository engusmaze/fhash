#![no_std]

use fhash::RandomState;
use hashbrown::HashSet;

fn main() {
    let bytes = b"Hello, World!";

    // Input your own seed from any other source of randomness
    let mut set: HashSet<&[u8], RandomState> = HashSet::with_hasher(RandomState::from(123));

    set.insert(bytes);

    // Do stuff with set
}
