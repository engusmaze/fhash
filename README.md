# FHash

FHash is a simple and lightweight hashing crate designed for flexibility, speed, and usability across various Rust environments. It provides a basic hashing algorithm, `FHasher`, along with a `RandomState` implementation intended for use in `HashMap` and `HashSet` structures. It's a direct competitor to `AHash` and is 1.3-2.0x faster.

## Features

### FHasher

- **Stupidly Simple**: Implements a straightforward hash function, offering basic yet effective hashing capabilities.
- **Minimalistic Design**: Designed to be lean and efficient, suitable for a wide range of hashing needs without unnecessary complexity.
- **Customizable**: Provides flexibility by allowing custom seeding for `FHasher` instances.

### RandomState

- **Enhanced Security**: Incorporates randomness into the hashing process, mitigating certain types of hash collision attacks and bolstering security in data structures.
- **Easy Integration**: Designed for seamless integration with `HashMap` and `HashSet` to enhance resilience against potential vulnerabilities in the hash function.

## Usage

### RandomState with HashMap/HashSet

```rust
use hashbrown::HashMap;
use fhash::RandomState;

let mut map: HashMap<i32, &str, RandomState> = HashMap::default();
map.insert(1, "apple");
map.insert(2, "banana");

assert_eq!(map.get(&1), Some(&"apple"));
assert_eq!(map.get(&2), Some(&"banana"));
```

### FHasher

```rust
use fhash::FHasher;

let mut hasher = FHasher::default();
hasher.write(b"Hello, world!");
let hash_result = hasher.finish();
```

## Compatibility

- **`no_std` Compatible**: Can be used in `no_std` environments or scenarios with limited access to the standard library.

## Contributing

Contributions to enhance FHash in terms of optimizations, features, or bug fixes are welcome! Please submit issues or pull requests.
