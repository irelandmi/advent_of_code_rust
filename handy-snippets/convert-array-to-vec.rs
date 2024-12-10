fn main() {
    let letters = ['a', 'b', 'c', 'd', 'e']; // Array
    let mut letters_vec: Vec<char> = letters.to_vec(); // Convert to vector

    letters_vec.push('f'); // Now we can add elements
    println!("{:?}", letters_vec); // ['a', 'b', 'c', 'd', 'e', 'f']
}
