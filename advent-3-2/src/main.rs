use std::fs;
use regex::Regex;
use std::io;

fn main() -> io::Result<()> {
    // Step 1: Read the file into a string
    let file_path = "input.txt"; // replace with your file path
    let contents = fs::read_to_string(file_path)?;

    // Step 2: Apply regex on the content of the file
    let re = Regex::new(r"(mul)\(([0-9]+),([0-9]+)\)").unwrap();

    let mut answer_vec: Vec<i32> = vec![];
    let mut final_answer: i32 = 0;
    
    // Regex to match "don't(...) do()" pattern
    let re_remove = Regex::new(r"(?s)(don't\(\)).*?(do\(\))").unwrap();

    // Replace matches with an empty string
    let final_contents = re_remove.replace_all(&contents, "").to_string();
    println!("{}",final_contents);
    for caps in re.captures_iter(&final_contents) {
        let mut answer: i32 = 0;
        let mut capture_answer: i32 = 0;
        let mut capture_answer_2: i32 = 0;
        if let Some(matched) = caps.get(2) { // Capture group 2
            let nums = matched.as_str();    
            if let Ok(num) = nums.parse::<i32>() {
                capture_answer = num;
            }
            
        }
        if let Some(matched) = caps.get(3) { // Capture group 3
            let nums = matched.as_str();           
            if let Ok(num) = nums.parse::<i32>() {
                capture_answer_2 = num;
            }
        }

        let answer: i32 = multiplier(capture_answer,capture_answer_2);
        println!("answer 1 {}",answer);
        answer_vec.push(answer);
    }
    
    println!("{:?}",answer_vec);
    final_answer = answer_vec.iter().sum();
    println!("{:?}",final_answer);
    Ok(())
}

fn multiplier(a: i32, b: i32) -> i32 {
    let c: i32;
    c = a * b;
    return c
}