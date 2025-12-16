
//use std::env;

mod day1;

fn main() {
    let path = "/workspaces/advent-of-code-2025-rust/day1-example.txt";
    // let path = "/workspaces/advent-of-code-2025-rust/day1-input.txt";
    let rotations = day1::read_file_lines(path).unwrap();
    let mut dial = day1::Dial::new();

    dial.perform_rotations(&rotations);

    // Output all positions
    println!("Positions: {:?}", dial.history);
    println!("Number of zero values: {}", dial.zero_count);
    println!("Number of zero crossings: {}", dial.zero_crossings);
}
