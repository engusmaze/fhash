use std::{
    fs::File,
    hint::black_box,
    io::Write,
    thread::{self},
    time::{Duration, Instant},
};

use fhash::DefaultFHasherBuilder;
use frand::Rand;
use hashbrown::HashSet;

const LENGTH: usize = 64;
const TRIES: usize = 100;
const ITERATIONS: usize = 100_000;

fn fhash() {
    let mut rng = Rand::new();

    let mut file = File::create("fhash").unwrap();
    for bytes_len in 0..=LENGTH {
        let mut duration = Duration::ZERO;
        let mut set = HashSet::with_hasher(DefaultFHasherBuilder);
        for _ in 0..TRIES {
            let bytes: Box<[u8]> = (0..bytes_len).map(|_| rng.gen()).collect();
            let start = Instant::now();
            for _ in 0..ITERATIONS {
                set.insert(black_box(bytes.clone()));
            }
            let end = Instant::now();
            duration += end - start;

            // sleep(Duration::from_millis(50));
        }
        writeln!(
            file,
            "{}",
            format!(
                "{:.4}",
                duration.as_secs_f64() / TRIES as f64 / ITERATIONS as f64 * 1_000_000_000.0
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
        let mut duration = Duration::ZERO;
        let mut set = HashSet::with_hasher(ahash::RandomState::new());
        for _ in 0..TRIES {
            let bytes: Box<[u8]> = (0..bytes_len).map(|_| rng.gen()).collect();
            let start = Instant::now();
            for _ in 0..ITERATIONS {
                set.insert(black_box(bytes.clone()));
            }
            let end = Instant::now();
            duration += end - start;

            // sleep(Duration::from_millis(50));
        }
        writeln!(
            file,
            "{}",
            format!(
                "{:.4}",
                duration.as_secs_f64() / TRIES as f64 / ITERATIONS as f64 * 1_000_000_000.0
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
