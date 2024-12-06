use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Array to hold the answers
    let mut answer_array = Vec::new();

    // Process each line in the file
    for line in reader.lines() {
        let line = line?; // Read the line as a string
        let numbers: Vec<i32> = line
            .split_whitespace() // Split by whitespace
            .filter_map(|s| s.parse::<i32>().ok()) // Parse as integers, ignoring errors
            .collect();

        println!("line: {:?}", numbers);

        let mut first_direction: i32 = 0;
        let mut direction: i32 = 0;

        for number in 0..numbers.len() - 1 {
            println!("index: {:?}", number);

            if number != line.len() {
                let previous_num: i32 = numbers[number];
                let next_num: i32 = numbers[number + 1];

                println!("previous num: {:?}", previous_num);
                println!("next num: {:?}", next_num);

                let diff: i32 = previous_num - next_num;
                println!("diff {:?}", diff.abs());

                if diff.abs() > 3 || diff.abs() == 0 {
                    answer_array.push(0);
                    println!("UNSAFE: HUGE DIFF");
                    break;
                }

                if diff > 0 {
                    direction = -1;
                } else {
                    direction = 1;
                }

                // Assess the direction of the difference
                if number == 0 {
                    first_direction = direction;
                }

                println!("first direction {:?}", first_direction);
                println!("direction {:?}", direction);

                if direction != first_direction {
                    answer_array.push(0);
                    println!("UNSAFE: different direction");
                    break;
                }

                if number == numbers.len() - 2 {
                    answer_array.push(1);
                }
            }
        }
    }

    println!("{:?}", answer_array);

    let answer: i32 = answer_array.iter().sum();
    println!("{:?}", answer);

    Ok(())
}
