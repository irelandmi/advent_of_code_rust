use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};


fn main() -> io::Result<()> {
    // Step 1: Create a HashMap to represent the grid
    let mut grid: HashMap<(i32, i32), String> = HashMap::new();
    // Open the file
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let valid_path = ["M","A","S"];
    let mut answer_array: Vec<i32> = vec![];

    // Process each line in the file
    let mut index: i32 = 0;
    let mut col: i32 = 0;

    for line_result in reader.lines() {
        let line = line_result?; // Read the line as a string
        println!("{}", line);

        for c in line.chars() {
            println!("c: {:?}",c);
            // Step 2: Populate the grid with some values
            grid.insert((index, col), c.to_string()); // Cell at (0, 0) has value 
            col = col + 1;
        }

        index = index + 1;
        col = 0;
    }
    parse_grid(&grid, valid_path, &mut answer_array);
    
    let total: i32 = answer_array.iter().sum();
    println!("Total Matches {:?}", total);
    Ok(())
}

fn generate_hash(input1: &str, input2: &str, input3: &str, input4: &str, input5: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    // Combine the inputs into a single hashable structure
    (input1, input2, input3).hash(&mut hasher);
    hasher.finish()
}

fn get_neighbours(target: (i32,i32)) -> [(i32, i32); 8] {
    // List the neighboring cell coordinates
    let neighbors = [
        (target.0 + 1, target.1 - 1), // bottom left
        (target.0 + 1, target.1 + 1), // bottom right
        (target.0 - 1, target.1 + 1), // top right
        (target.0 - 1, target.1 - 1), // top left
    ];
    return neighbors
}

fn compute_valid_combinations(valid_path: [&str, 3]) -> Vec<u64> {
    let up: i32 = -1;
    let down: i32 = 1;
    let left: i32 = -1;
    let right: i32 = 1;

    let checker: Vec<u32> = vec![];

    let top_left_bottom_right: Vec<((i32,i32),(i32,i32),(i32,i32))> = Vec<((-1, -1), (0, 0), (1, 1))>;
    let bottom_right_top_left: Vec<((1, 1), (0, 0), (-1, -1))>;
    let bottom_left_top_right: Vec<((1, -1), (0, 0), (-1, 1))>;
    let top_right_bottom_left: Vec<((-1, 1), (0, 0), (1, -1))>;

    for i in arrays {
        check_if_valid
            checker.push(1);
        check_if_valid_reversed
            checker.push(1);
        if checker.sum() == 2 {
            return 1
        }
    }
    return 0
}

fn parse_grid(grid: &HashMap<(i32, i32), String>, valid_path: [&str; 4], answer_array: &mut Vec<i32>) {
    for (key, value) in grid {   
        let origin: &(i32, i32) = &key.clone();   
        println!("Origin: {:?}", origin);  
        if value == valid_path[0] {
            follow_path(*origin, valid_path, grid, 1, answer_array);
        }
    }
}

fn follow_path (origin: (i32, i32), valid_path: [&str; 4], grid: &HashMap<(i32, i32), String>, position: usize, answer_array: &mut Vec<i32>) {
    let t_neighbours = get_neighbours(origin);
    println!("neighbours {:?}", t_neighbours);
    for neighbour in t_neighbours {
        println!("neighbour: {:?}",neighbour);
        if let Some(value) = grid.get(&neighbour) {
            if *value == valid_path[position] {
                println!("current_value {}",value);
                let dir: (i32, i32) = (
                    neighbour.0 - origin.0,
                    neighbour.1 - origin.1
                );
                follow_path_direction(origin, dir, valid_path, grid, answer_array);
            }
        } else {
            println!("No value found for neighbour: {:?}", neighbour);
        }
    }
}

fn follow_path_direction(origin: (i32, i32), dir: (i32,i32), valid_path: [&str; 4], grid: &HashMap<(i32, i32), String>, answer_array: &mut Vec<i32>) {
    let mut locations: Vec<(i32,i32)> = vec![];
    locations.push(origin);
    println!("Pushing Origin: {:?}",origin);
    let mut new_location: (i32, i32) = origin;
    let mut new_location_answers: Vec<Option<&String>> = vec![];

    for i in 1..valid_path.len() {
        new_location = (new_location.0 + dir.0, new_location.1 + dir.1);
        locations.push(new_location);
        
    }
    println!("Locations: {:?}",locations);
    for loc in &locations {
        // Push the result of grid.get() into new_location_answers
        new_location_answers.push(grid.get(loc));
    }
    println!("Checking locations {:?}",new_location_answers);

    for (index, value) in new_location_answers.iter().enumerate() {
        println!("index {:?}, value {:?}", index, value);
        println!("valid path index {:?}", valid_path[index]);

        if Some(String::from(valid_path[index])) == value.as_deref().cloned() {
            println!("its a match!");
        }
        else {
            break
        }
        
        if value.as_deref() == Some(&String::from(valid_path[valid_path.len()-1])) && index == valid_path.len()-1{
            println!("last item!");
            answer_array.push(1);
        }
    }
}