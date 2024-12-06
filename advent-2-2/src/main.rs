use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // Array to hold the answers
    let mut answer_array = Vec::<i32>::new();

    // Process each line in the file
    for line in reader.lines() {
        let numbers: Vec<i32> = line?
            .split_whitespace() // Split by whitespace
            .filter_map(|s| s.parse::<i32>().ok()) // Parse as integers, ignoring errors
            .collect();
        error_handler(numbers, &mut answer_array);
    }
    println!("{:?}", answer_array);
    let answer: i32 = answer_array.iter().sum();
    println!("{:?}", answer);
    Ok(())
}

fn create_subvectors<T: Clone>(vec: &Vec<T>) -> Vec<Vec<T>> {
    let mut result = Vec::new();

    for i in 0..vec.len() {
        let mut new_vec = vec.clone(); // Create a copy of the original vector
        new_vec.remove(i);            // Remove the element at index `i`
        result.push(new_vec);         // Add the new vector to the result
    }

    return result
}

fn damage_control_error(vec: Vec<i32>, answer_array: &mut Vec<i32>) {
    
    // for number in (0..vec.len()).rev() {
    //     let mut vec_copy = vec.clone(); // Clone the vector to make a copy
    //     vec_copy.remove(number);
    // }
    //     // fn (whatever)
    // return Some(1)
    let mut svecs = create_subvectors(&vec);
    for svec in svecs {
        error_handler(svec, answer_array);
    }
    answer_array.push(0);
    println!("UNSAFE: HUGE DIFF");
}

fn error_handler(numbers: Vec<i32>) {

    println!("line: {:?}", numbers);

    let mut first_direction: i32 = 0;
    let mut direction: i32 = 0;

    for number in 0..numbers.len() - 1 {
        if number != numbers.len() {
            let previous_num: i32 = numbers[number];
            let next_num: i32 = numbers[number + 1];

            println!("previous num: {:?}", previous_num);
            println!("next num: {:?}", next_num);

            let diff: i32 = previous_num - next_num;
            println!("diff {:?}", diff.abs());

            if diff.abs() > 3 || diff.abs() == 0 {
                println!("UNSAFE: Large difference");
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
                println!("UNSAFE: different direction");
                break;
            }

            if number == numbers.len() - 2 {
                answer_array.push(1);
            }
        }
    }
}