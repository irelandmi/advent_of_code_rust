use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

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

    
    // Count occurrences of each element in array_b
    let mut counts = HashMap::new();
    for &item in &second_array {
        *counts.entry(item).or_insert(0) += 1;
    }

    let mut similarity_score: Vec<i32> = vec![];
    // Compare elements in array_a with counts in array_b
    for &item in &first_array {
        let count = counts.get(&item).unwrap_or(&0);
        println!("Element {} occurs {} time(s) in second_array", item, count);
        let total_ss = item * count;
        similarity_score.push(total_ss); 
    }

    println!("Total Similarity Score {:?}", similarity_score);

    let total: i32 = similarity_score.iter().sum();

    println!("Total Differences {:?}", total);

    Ok(())
}