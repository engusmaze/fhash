use std::hash::Hasher;

use fhash::FHasher;

fn main() {
    for i in 0..64 {
        let arr: Box<[u8]> = (0..i).map(|_| 0u8).collect();
        let mut hasher = FHasher::default();
        hasher.write(&arr);
        println!("{}", hasher.finish());
    }
}
