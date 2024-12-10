use std::array;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};


fn main() -> io::Result<()> {
    // Step 1: Create a HashMap to represent the grid
    let mut grid: HashMap<(i32, i32), String> = HashMap::new();
    // Open the file
    let file = File::open("test_input.txt")?;
    let reader = BufReader::new(file);
    let valid_path = ["X","M","A","S"];
    let mut answer_array: Vec<i32> = vec![0];

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
    parse_grid(grid, valid_path);

    Ok(())
}       

fn get_neighbours(target: (i32,i32)) -> [(i32, i32); 8] {
    // List the neighboring cell coordinates
    let neighbors = [
        (target.0 + 1, target.1),            // below
        (target.0 + 1, target.1 - 1), // bottom left
        (target.0 + 1, target.1 + 1),            // bottom right
        (target.0, target.1 - 1), // left
        (target.0, target.1 + 1),            // right
        (target.0 - 1, target.1), // above
        (target.0 - 1, target.1 + 1), // top right
        (target.0 - 1, target.1 - 1), // top left
    ];
    return neighbors
    // answer_array.push(0)
}

fn parse_grid(grid: HashMap<(i32, i32), String>, valid_path: [&str; 4]) {
    for (key, value) in &grid {   
        let origin: &(i32, i32) = &key.clone();   
        println!("Origin: {:?}", origin);  
        if value == valid_path[0] {
            follow_path(*origin, valid_path, grid.clone());
        }
    }
}

fn follow_path (origin: (i32, i32), valid_path: [&str; 4], grid: HashMap<(i32, i32), String>) {
    let t_neighbours = get_neighbours(origin);
    for i in t_neighbours {
        println!("neighbour: {:?}",i);
        if let Some(value) = grid.get(&i) {
            if value == valid_path[1] {
                let path: [(i32, i32); 4];
                let dif: (i32, i32);
                dif = (
                    origin.0 - i.0,
                    origin.1 - i.1
                );
                // println!("diff: {:?}", dif);
                // let remaining_path_len: &usize = &valid_path[2..].len();
                // let path_locations: [(i32,i32); remaining_path_len];
                // for p in &valid_path[2..] {
                    
                // }
                // for p in &valid_path[2..] {
                    
                // }
            }
        } else {
            println!("No value found for neighbour: {:?}", i);
        }
        // for p in valid_path {
        //     let dir: i32 = origin - i
        // }
    }
}

fn get_paths(origin: (i32,i32), size: i32) -> Vec<i32> {
    let t_origin: (i32, i32) = origin.clone();
    let n_path: (i32, i32);
    let diff: (i32, i32);
    diff = (
        origin.0 - origin.0,
        origin.1 - origin.1
    );
    println!("diff: {:?}", diff);
    for i in 1..size-1 {
        n_path = (
            origin.0 + diff.0,
            origin.1 + diff.1
        )
    }
    return 
}
// fn map_dir(dir: (i32,i32)) -> [(i32,i32); 4] {
//     let targets 

//     return targets
// }

//     for i in neighbours
//     let origin = (0, 0);
//     get_neighbours(target);
//     for &(row, col) in &neighbors {
//         if let Some(&value) = grid.get(&(row, col)) {
//             println!("Neighbor at ({}, {}): {}", row, col, value);
//         } else {
//             println!("Neighbor at ({}, {}): None", row, col);
//         }
//     }
// }

// fn valid_path(previous_path: str, valid_path: str, ) {
//     if previous_path == valid_path {
//         return true;
//     }
//     return false;
// }

// fn follow_path (&origin: ) {

//     for i in neighbours
//     let origin = (0, 0);
//     get_neighbours(target);
//     for &(row, col) in &neighbors {
//         if let Some(&value) = grid.get(&(row, col)) {
//             println!("Neighbor at ({}, {}): {}", row, col, value);
//         } else {
//             println!("Neighbor at ({}, {}): None", row, col);
//         }
//     }
// }