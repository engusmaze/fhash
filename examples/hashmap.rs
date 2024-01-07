use fhash::RandomState;
use hashbrown::HashMap;

fn main() {
    let mut map: HashMap<i32, &str, RandomState> = HashMap::default();

    // Inserting values into the HashMap
    map.insert(1, "apple");
    map.insert(2, "banana");

    println!("{map:?}");
}
