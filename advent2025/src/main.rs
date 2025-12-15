
//use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Get file path from command-line arguments
    // let path = env::args().nth(1).expect("Usage: cargo run -- <file_path>");

    let path = "/workspaces/advent-of-code-2025-rust/day1-example.txt";

    // Open the file
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut position: i32 = 50;
    let mut positions: Vec<i32> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue; // Skip empty lines
        }

        // First character determines direction
        let (direction, value_str) = trimmed.split_at(1);
        let value: i32 = value_str.trim().parse().expect("Invalid number");

        match direction {
            "L" => position -= value,
            "R" => position += value,
            _ => panic!("Invalid line format: {}", trimmed),
        }

        // Apply modulo 100 (ensure positive result)
        position = ((position % 100) + 100) % 100;

        positions.push(position);
    }

    // Output all positions
    println!("Positions: {:?}", positions);
    
    let zero_count = positions.iter().filter(|&&x| x == 0).count();
    println!("Number of zero values: {}", zero_count);

    Ok(())
}
