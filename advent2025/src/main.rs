
//use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let path = "/workspaces/advent-of-code-2025-rust/day1-example.txt";
    // let path = "/workspaces/advent-of-code-2025-rust/day1-input.txt";

    // Open the file
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut dial = Dial::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue; // Skip empty lines
        }

        // First character determines direction
        let (direction, value_str) = trimmed.split_at(1);
        let clicks: i32 = value_str.trim().parse().expect("Invalid number");

        match direction {
            "L" => dial.turn_left(clicks),
            "R" => dial.turn_right(clicks),
            _ => panic!("Invalid line format: {}", trimmed),
        }
    }

    // Output all positions
    println!("Positions: {:?}", dial.history);
    println!("Number of zero values: {}", dial.zero_count);

    Ok(())
}


struct Dial {
    position: i32,
    history: Vec<i32>,
    zero_count: i32,
}

impl Dial {
    /// Creates a new Dial with position initialized to 50.
    fn new() -> Self {
        Self {
            position: 50,
            history: vec![50],
            zero_count: 0,
        }
    }

    fn turn_left(&mut self, clicks: i32) {
        self.position = (self.position - clicks).rem_euclid(100);
        self.history.push(self.position);
        if self.position == 0 {
            self.zero_count += 1;
        }

    }

    fn turn_right(&mut self, clicks: i32) {
        self.position = (self.position + clicks).rem_euclid(100);
        self.history.push(self.position);
        if self.position == 0 {
            self.zero_count += 1;
        }
    }

    fn position(&self) -> i32 {
        self.position
    }

    fn history(&self) -> &[i32] {
        &self.history
    }

}
