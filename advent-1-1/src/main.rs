use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Arrays to hold the numbers
    let mut first_array = Vec::new();
    let mut second_array = Vec::new();

    // Process each line in the file
    for line in reader.lines() {
        let line = line?; // Read the line as a string
        let numbers: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .filter_map(|s| s.parse::<i32>().ok()) // Parse as integers, ignoring errors
            .collect();

        // Check if there are at least two numbers in the line
        if numbers.len() >= 2 {
            first_array.push(numbers[0]); // First number goes to first_array
            second_array.push(numbers[1]); // Second number goes to second_array
        }
    }

    first_array.sort();
    second_array.sort();

        // Calculate the differences
        let differences: Vec<i32> = first_array
        .iter() // Create an iterator over the first array
        .zip(second_array.iter()) // Pair elements from the second array
        .map(|(a, b)| (a - b).abs()) // Calculate absolute differences
        .collect(); // Collect results into a vector

    // Print the differences
    println!("Differences: {:?}", differences);
    let total: i32 = differences.iter().sum();

    println!("Total Differences {:?}", total);

    Ok(())
}