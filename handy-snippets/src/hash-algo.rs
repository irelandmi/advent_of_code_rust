use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn generate_hash(input1: &str, input2: &str, input3: &str, input4: &str, input5: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    // Combine the inputs into a single hashable structure
    (input1, input2, input3, input4, input5).hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let hash = generate_hash("Hello", "World", "Hash", "Rust", "Example");
    println!("Generated Hash: {}", hash);
}
