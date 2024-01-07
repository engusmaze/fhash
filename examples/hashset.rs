use fhash::RandomState;
use hashbrown::HashSet;

fn main() {
    let mut set: HashSet<&str, RandomState> = HashSet::default();

    // Inserting values into the HashMap
    set.insert("apple");
    set.insert("banana");

    println!("{set:?}");
}
