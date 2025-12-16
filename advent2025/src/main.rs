
//use std::env;
use std::fs;
use std::io;
use std::io::BufRead;

fn main() {
    // let path = "/workspaces/advent-of-code-2025-rust/day1-example.txt";
    let path = "/workspaces/advent-of-code-2025-rust/day1-input.txt";
    let rotations = read_file_lines(path).unwrap();
    let mut dial = Dial::new();

    dial.perform_rotations(&rotations);

    // Output all positions
    println!("Positions: {:?}", dial.history);
    println!("Number of zero values: {}", dial.zero_count);
    println!("Number of zero crossings: {}", dial.zero_crossings);
}

fn read_file_lines(path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}



struct Dial {
    position: i32,
    history: Vec<i32>,
    zero_count: i32,
    zero_crossings: i32,
}

impl Dial {
    /// Creates a new Dial with position initialized to 50.
    fn new() -> Self {
        Self {
            position: 50,
            history: vec![50],
            zero_count: 0,
            zero_crossings: 0,
        }
    }

    fn perform_rotations(&mut self, rotations: &Vec<String>) {
        for line in rotations {
            // First character determines direction
            let (direction, value_str) = line.split_at(1);
            let clicks: i32 = value_str.trim().parse().expect("Invalid number");

            match direction {
                "L" => self.turn_left(clicks),
                "R" => self.turn_right(clicks),
                _ => panic!("Invalid line format: {}", line),
            }
        }
    }

    fn turn_left(&mut self, clicks: i32) {
        self.zero_crossings += (clicks + 100 - self.position) / 100;
        self.turn(-clicks);
    }

    fn turn_right(&mut self, clicks: i32) {
        self.zero_crossings += (clicks + self.position) / 100;
        self.turn(clicks);
    }

    fn turn(&mut self, clicks: i32) {
        self.position = (self.position - clicks).rem_euclid(100);
        self.history.push(self.position);
        if self.position == 0 {
            self.zero_count += 1;
        }
    }
}
