use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use crate::utils::read_file_lines;
// use rstest::rstest;

#[allow(dead_code)]
pub fn main() {
    let path = "/workspaces/advent-of-code-2025-rust/day10-example.txt";
    // let path = "/workspaces/advent-of-code-2025-rust/day10-input.txt";

    let lines = read_file_lines(path).unwrap();
    let mut total_presses = 0;
    for line in lines {
        let presses = find_fewest_button_presses(line);
        println!("Fewest button presses: {}", presses);
        println!();
        total_presses += presses;
    }

    println!("Total presses: {}", total_presses);
}

fn find_fewest_button_presses(machine_description: String) -> u32 {
    let parts =  machine_description.split_once("]").unwrap();
    let desired_state = find_desired_state(parts.0);
    println!("Desired State: {:?}", desired_state);
    let (line_fragment_2, line_fragment_3) = parts.1.split_once("{").unwrap();
    let masks: Vec<u8> = line_fragment_2
        .split_whitespace()
        .map(|s| 
            s.trim_matches(|c| c == '(' || c == ')')
            .split(",")
            .map(|i| 1u8<<i.parse::<usize>().unwrap())
            .sum()
        )
        .collect();
    println!("Steps {:?}", masks);
    println!("Joltages (unused) {:?}", line_fragment_3);
    let mut universes: BinaryHeap<(Reverse<u32>, u8, Vec<u8>)> = BinaryHeap::new();
    universes.push((Reverse(0), 0, Vec::new()));
    let mut states_seen: HashSet<u8> = HashSet::new();
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
        for mask in &masks {
            let new_state = universe.1 ^ mask;
            let mut new_steps = universe.2.clone();
            new_steps.push(*mask);
        
        
            // Don't revisit the same state twice
            if !states_seen.contains(&new_state) {
                let new_universe = (Reverse(new_priority), new_state, new_steps);
                if new_state == desired_state {
                    println!("Target reached: {:?}", new_universe);
                    return new_priority;
                };
                universes.push(new_universe);
                states_seen.insert(new_state);                
            }
        }
    }

    panic!("Desired state not found!");
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