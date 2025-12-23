use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use crate::utils::read_file_lines;
use rstest::rstest;

#[allow(dead_code)]
pub fn main() {
    // let path = "/workspaces/advent-of-code-2025-rust/day10-example.txt";
    let path = "/workspaces/advent-of-code-2025-rust/day10-input.txt";

    let total_presses = part1(path);

    println!("Total presses: {}", total_presses);
}

fn part1(path: &str) -> u32 {
    let lines = read_file_lines(path).unwrap();

    let mut total_presses = 0;
    for line in lines {
        let parts =  line.split_once("]").unwrap();
        let desired_state = find_desired_state(parts.0);
        println!("Desired State: {:?}", desired_state);
        let (line_fragment_2, line_fragment_3) = parts.1.split_once("{").unwrap();
        let button_actions = find_button_actions(line_fragment_2);
        println!("Steps {:?}", button_actions);
        println!("Joltages (unused) {:?}", line_fragment_3);


        let presses = find_fewest_button_presses(desired_state, button_actions);
        println!("Fewest button presses: {}", presses);
        println!();
        total_presses += presses;
    }

    total_presses
}

fn find_button_actions(line_fragment_2: &str) -> Vec<Vec<usize>> {
    line_fragment_2
        .split_whitespace()
        .map(|s| 
            s.trim_matches(|c| c == '(' || c == ')')
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect()
        )
        .collect()
}

fn find_fewest_button_presses(desired_state: Vec<u16>, buttons: Vec<Vec<usize>>) -> u32 {
    let mut universes: BinaryHeap<(Reverse<u32>, Vec<u16>)> = BinaryHeap::new();
    universes.push((Reverse(0), vec![0; desired_state.len()]));
    let mut states_seen: HashSet<String> = HashSet::new();
    loop {
        // Get the universe with the lowsest number of button pushes so far.
        let universe = match universes.pop() {
            Some(u) => u,
            None => { break; },
        };
        println!("Universe: {:?}", universe);

        let Reverse(priority) = universe.0;
        let new_priority = priority + 1;

        // Spawn new universes for each possible mask.
        for mask in &buttons {
            // Update state
            let mut new_state = universe.1.clone();
            for element in mask {
                new_state[*element] ^= 1; // Toggle state
            }

            // Don't revisit the same state twice
            let state = format!("{:?}", new_state);
            if !states_seen.contains(&state) {
                let new_universe = (Reverse(new_priority), new_state.clone());

                if new_state == desired_state {
                    println!("Target reached: {:?}", new_universe);
                    return new_priority;
                };

                universes.push(new_universe);
                states_seen.insert(state);                
            }
        }
    }

    panic!("Desired state not found!");
}

fn find_desired_state(first_line_fragment: &str) -> Vec<u16> {
    first_line_fragment
        .chars()
        .skip(1)
        .map(|c| match c {
            '.' => 0u16,
            '#' => 1u16,
             _ => panic!()
        })
        .collect()
}


#[rstest]
#[case("/workspaces/advent-of-code-2025-rust/day10-example.txt", 7)]
#[case("/workspaces/advent-of-code-2025-rust/day10-input.txt", 461)]
fn test_part1_answers(
    #[case] path: &str,
    #[case] expected_presses: u32
)
{
    // Act
    let presses = part1(path);

    // Assert
    assert_eq!(presses, expected_presses);
}