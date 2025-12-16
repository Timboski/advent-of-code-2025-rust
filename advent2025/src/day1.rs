use std::fs;
use std::io;
use std::io::BufRead;


pub fn read_file_lines(path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}



pub struct Dial {
    pub position: i32,
    pub history: Vec<i32>,
    pub zero_count: i32,
    pub zero_crossings: i32,
}

impl Dial {
    /// Creates a new Dial with position initialized to 50.
    pub fn new() -> Self {
        Self {
            position: 50,
            history: vec![50],
            zero_count: 0,
            zero_crossings: 0,
        }
    }

    pub fn perform_rotations(&mut self, rotations: &Vec<String>) {
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
        // Special case if dial is on zero, adding 100 causes an additional crossing.
        self.zero_crossings += match self.position {
            0 => clicks / 100,
            _ => (clicks + 100 - self.position) / 100,
        };
        self.turn(-clicks);
    }

    fn turn_right(&mut self, clicks: i32) {
        self.zero_crossings += (clicks + self.position) / 100;
        self.turn(clicks);
    }

    fn turn(&mut self, clicks: i32) {
        self.position = (self.position + clicks).rem_euclid(100);
        self.history.push(self.position);
        if self.position == 0 {
            self.zero_count += 1;
        }
    }
}

#[test]
fn problem1_example() {
    // Arrange
    let path = "/workspaces/advent-of-code-2025-rust/day1-example.txt";
    let rotations = read_file_lines(path).unwrap();
    let mut dial = Dial::new();

    // Act
    dial.perform_rotations(&rotations);

    // Assert
    assert_eq!(dial.zero_count, 3);
}

#[test]
fn problem1_real_data() {
    // Arrange
    let path = "/workspaces/advent-of-code-2025-rust/day1-input.txt";
    let rotations = read_file_lines(path).unwrap();
    let mut dial = Dial::new();

    // Act
    dial.perform_rotations(&rotations);

    // Assert
    assert_eq!(dial.zero_count, 1059);
}

#[test]
fn problem2example() {
    // Arrange
    let path = "/workspaces/advent-of-code-2025-rust/day1-example.txt";
    let rotations = read_file_lines(path).unwrap();
    let mut dial = Dial::new();

    // Act
    dial.perform_rotations(&rotations);

    // Assert
    assert_eq!(dial.zero_crossings, 6);
}

#[test]
fn problem2real_data() {
    // Arrange
    let path = "/workspaces/advent-of-code-2025-rust/day1-input.txt";
    let rotations = read_file_lines(path).unwrap();
    let mut dial = Dial::new();

    // Act
    dial.perform_rotations(&rotations);

    // Assert
    println!("Number of zero crossings: {}", dial.zero_crossings);
    assert!(dial.zero_crossings < 6315); // First guess was too high
    assert_eq!(dial.zero_crossings, 6305);
}

#[test]
fn given_dial_on_zero_when_rotate_left_expect_correct_number_of_zero_crossings() {
    // Arrange
    let mut dial = Dial::new();
    dial.position = 0; // Hack the start position

    // Act
    dial.turn_left(0);

    // Assert
    assert_eq!(dial.zero_crossings, 0);
}