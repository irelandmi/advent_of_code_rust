// using a for loop
fn main() {
    let vector = vec![10, 20, 30, 40];

    for value in &vector {
        println!("Value: {}", value);
    }
}

// iterator
fn main() {
    let vector = vec![10, 20, 30, 40];

    vector.iter().for_each(|value| {
        println!("Value: {}", value);
    });
}

// mutable reference
fn main() {
    let mut vector = vec![10, 20, 30, 40];

    for value in &mut vector {
        *value += 1; // Modify each element
    }

    println!("{:?}", vector); // Output: [11, 21, 31, 41]
}

// while loop with an index
fn main() {
    let vector = vec![10, 20, 30, 40];
    let mut index = 0;

    while index < vector.len() {
        println!("Value at {}: {}", index, vector[index]);
        index += 1;
    }
}

// Using enumerate
fn main() {
    let vector = vec![10, 20, 30, 40];

    for (index, value) in vector.iter().enumerate() {
        println!("Index: {}, Value: {}", index, value);
    }
}
