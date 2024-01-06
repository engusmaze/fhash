use std::{
    fs::File,
    hash::Hasher,
    hint::black_box,
    io::Write,
    thread,
    time::{Duration, Instant},
};

use ahash::AHasher;
use fhash::FHasher;
use frand::Rand;

const LENGTH: usize = 128;
const ITERATIONS: usize = 4_000_000;

fn fhash() {
    let mut rng = Rand::new();

    let mut file = File::create("fhash").unwrap();
    for bytes_len in 0..=LENGTH {
        let bytes: Box<[u8]> = (0..bytes_len).map(|_| rng.gen()).collect();
        let mut duration = Duration::ZERO;
        for _ in 0..10 {
            let start = Instant::now();
            for _ in 0..ITERATIONS {
                let mut hasher = FHasher::default();
                hasher.write(black_box(&bytes));
                black_box(hasher.finish());
                // black_box(mix_with_bytes(0, black_box(&bytes)));
            }
            let end = Instant::now();
            duration = duration + (end - start);
        }
        writeln!(
            file,
            "{}",
            format!(
                "{:.4}",
                duration.as_secs_f64() / 10.0 / ITERATIONS as f64 * 1_000_000_000.0
            )
            .replace('.', ",")
        )
        .unwrap();
    }
}
fn ahash() {
    let mut rng = Rand::new();

    let mut file = File::create("ahash").unwrap();
    for bytes_len in 0..=LENGTH {
        let bytes: Box<[u8]> = (0..bytes_len).map(|_| rng.gen()).collect();
        let mut duration = Duration::ZERO;
        for _ in 0..10 {
            let start = Instant::now();
            for _ in 0..ITERATIONS {
                let mut hasher = AHasher::default();
                hasher.write(black_box(&bytes));
                black_box(hasher.finish());
            }
            let end = Instant::now();
            duration = duration + (end - start);
        }
        writeln!(
            file,
            "{}",
            format!(
                "{:.4}",
                duration.as_secs_f64() / 10.0 / ITERATIONS as f64 * 1_000_000_000.0
            )
            .replace('.', ",")
        )
        .unwrap();
    }
}

fn main() {
    for thread in [thread::spawn(fhash), thread::spawn(ahash)] {
        thread.join().unwrap();
    }
}
