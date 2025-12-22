use crate::utils::read_file_lines;
// use rstest::rstest;

#[allow(dead_code)]
pub fn main() {
    let path = "/workspaces/advent-of-code-2025-rust/day10-example.txt";
    // let path = "/workspaces/advent-of-code-2025-rust/day10-input.txt";

    // Do it exhaustively to check against example
    let lines = read_file_lines(path).unwrap();
    // let line = lines.iter().skip(1).next().unwrap();
    let line = lines.first().unwrap();

    let parts =  line.split_once("]").unwrap();
    let desired_state = find_desired_state(parts.0);
    println!("Desired State: {:?}", desired_state);
    let (line_fragment_2, line_fragment_3) = parts.1.split_once("{").unwrap();
    
    let mask: Vec<u8> = line_fragment_2
        .split_whitespace()
        .map(|s| 
            s.trim_matches(|c| c == '(' || c == ')')
            .split(",")
            .map(|i| 1u8<<i.parse::<usize>().unwrap())
            .sum()
        )
        .collect();
    println!("Steps {:?}", mask);

    // Test applying steps.
    let mut state = 0u8;
    //let steps = vec![0, 1, 2];
    let steps = vec![1, 3, 5, 5];
    for step in steps {
        state ^= mask[step];
        println!("Applying {} => {}", step, state);
    }
    
    
    println!("Joltages (unused) {:?}", line_fragment_3);
}

fn find_desired_state(first_line_fragment: &str) -> u8 {
    first_line_fragment
        .chars()
        .skip(1)
        .collect::<Vec<char>>()
        .iter()
        .rev()
        .map(|c| match c {
            '.' => 0u8,
            '#' => 1u8,
             _ => panic!()
        })
        .enumerate()
        .map(|(i, v)| v<<i)
        .into_iter()
        .sum()
}