use std::cmp::Ordering;
use std::collections::BinaryHeap;
use crate::utils::read_file_lines;
use rstest::rstest;

#[allow(dead_code)]
pub fn main() {
    let path = "/workspaces/advent-of-code-2025-rust/day10-example.txt";
    // let path = "/workspaces/advent-of-code-2025-rust/day10-input.txt";

    let total_presses_for_lights = part1(path);
    // let total_presses_for_joltage = part2(path);

    println!("Total presses for lights: {}", total_presses_for_lights);
    //println!("Total presses for joltage: {}", total_presses_for_joltage);
}

fn part1(path: &str) -> u32 {
    let lines = read_file_lines(path).unwrap();

    let mut total_presses = 0;
    for line in lines {
        let initial_state = StatePart1::new(line.as_str());
        let boxed = Box::new(initial_state);

        let presses = find_fewest_button_presses(boxed);

        println!("Fewest button presses: {}", presses);
        println!();
        total_presses += presses;
    }

    total_presses
}

// fn part2(path: &str) -> u32 {
//     let lines = read_file_lines(path).unwrap();

//     let mut total_presses = 0;
//     for line in lines {
//         let parts =  line.split_once("]").unwrap();
//         println!("Lights (unused): {:?}", parts.0);
//         let (line_fragment_2, line_fragment_3) = parts.1.split_once("{").unwrap();
//         let mut button_actions = find_button_actions(line_fragment_2);
//         println!("Steps {:?}", button_actions);
//         let joltages: Vec<u16> = line_fragment_3.trim_end_matches("}").split(",").map(|s| s.parse().unwrap()).collect();
//         println!("Joltages {:?}", joltages);

//         // Sort button actions by the total increase in joltage (number of elements)
//         button_actions.sort_by_key(|e| e.len());
//         button_actions.reverse();
//         println!("Sorted Steps {:?}", button_actions);


//         let behaviour = BehaviourPart2;
//         let presses = find_fewest_button_presses(joltages, button_actions, &behaviour);
//         println!("Fewest button presses: {}", presses);
//         println!();
//         total_presses += presses;
//     }

//     total_presses
// }

// fn find_button_actions(line_fragment_2: &str) -> Vec<Vec<usize>> {
//     line_fragment_2
//         .split_whitespace()
//         .map(|s| 
//             s.trim_matches(|c| c == '(' || c == ')')
//             .split(",")
//             .map(|s| s.parse::<usize>().unwrap())
//             .collect()
//         )
//         .collect()
// }

fn find_fewest_button_presses(initial_state: Box<dyn State>) -> u32 {
    let mut universes: BinaryHeap<ByKey> = BinaryHeap::new();
    universes.push(ByKey(initial_state));

    loop {
        // Get the universe with the lowsest number of button pushes so far.
        let ByKey(universe) = match universes.pop() {
            Some(u) => u,
            None => { break; },
        };
        println!("\rUniverses: {} Current Universe: {}", universes.len(), universe.display());

        // Spawn new universes for each possible next state.
        for child in universe.next_states() {
            if child.is_desired_state() {
                println!("Target reached: {}", child.button_pushes());
                return child.button_pushes();
            };

            if child.is_valid_state() { universes.push(ByKey(child)) }
        }
    }

    panic!("Desired state not found!");
}

// fn find_desired_state(first_line_fragment: &str) -> Vec<u16> {
//     first_line_fragment
//         .chars()
//         .skip(1)
//         .map(|c| match c {
//             '.' => 0u16,
//             '#' => 1u16,
//              _ => panic!()
//         })
//         .collect()
// }


trait State {
    fn next_states(&self) -> Vec<Box<dyn State>>;
    fn is_valid_state(&self)->bool;
    fn priority(&self)->i32;
    fn button_pushes(&self)->u32;
    fn is_desired_state(&self)->bool;
    fn display(&self)->String;

}

struct StatePart1 {
    button_pushes: u32,
    state: u16,
    desired_state: u16,
    masks: Vec<u16>
}

impl StatePart1 {
    fn new(machine_definition: &str) -> Self {
        let (state_def, remainder) = machine_definition.split_once("]").unwrap();
        let (maks_def, joltages) = remainder.split_once("{").unwrap();

        // Createa a bitmap for the desired state
        let desired_state = state_def
            .chars()
            .skip(1)
            .map(|c| match c {
                '.' => 0u16,
                '#' => 1u16,
                _ => panic!()
            })
            .enumerate()
            .map(|(i, v)| v << i)
            .sum();
        println!("Desired State: {}={:0b}", desired_state, desired_state);

        // Store the button pushes as bit masks
        let button_actions = maks_def
            .split_whitespace()
            .map(|s| 
                s.trim_matches(|c| c == '(' || c == ')')
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .map(|i| 1 << i)
                .sum()
            )
            .collect();
        println!("Button Actions: {:?}", button_actions);
        print!("BitMasks:");
        for m in &button_actions { print!(" {:b}", m )}
        println!();

        println!("Joltages (unused): {:?}", joltages);

        Self {
            button_pushes: 0,
            state: 0,
            desired_state: desired_state,
            masks: button_actions
        }
    }

    fn from_parent(parent: &StatePart1, mask: &u16) -> Self {
        Self {
            button_pushes: parent.button_pushes + 1,
            state: parent.state ^ mask,
            desired_state: parent.desired_state,
            masks: parent.masks.clone() // TODO: Point to one copy of this
        }
    }
}

// Implement ordering so BinaryHeap compares only `priority`.

struct ByKey(Box<dyn State>);

impl Eq for ByKey {}
impl PartialEq for ByKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.priority() == other.0.priority()
    }
}
impl Ord for ByKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // Max-heap behavior: larger key comes first
        self.0.priority().cmp(&other.0.priority())
    }
}
impl PartialOrd for ByKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl State for StatePart1 {
    fn is_valid_state(&self)->bool { true }
    
    fn next_states(&self)->Vec<Box<dyn State>> {
        self.masks.iter()
            .map(|mask| Box::new(StatePart1::from_parent(self, mask)) as Box<dyn State>)
            .collect()
    }
    
    fn priority(&self)->i32 { -(self.button_pushes as i32) }
    fn button_pushes(&self)->u32 { self.button_pushes }
    fn is_desired_state(&self)->bool { self.state == self.desired_state }
    fn display(&self)->String { format!("{}", self.state) }
}

// struct BehaviourPart2;
// impl State for BehaviourPart2 {
//     fn update_state(&self, s: u16)->u16 { s + 1 }

//     fn is_valid_state(&self, state: &Vec<u16>, desired_state: &Vec<u16>)->bool {
//         !state.iter().zip(desired_state).any(|(n, d)| n > d)
//     }
    
//     fn log_frequency(&self)->u32 { 10000 }
// }


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

// #[rstest]
// #[case("/workspaces/advent-of-code-2025-rust/day10-example.txt", 33)]
// //#[case("/workspaces/advent-of-code-2025-rust/day10-input.txt", ??)]
// fn test_part2_answers(
//     #[case] path: &str,
//     #[case] expected_presses: u32
// )
// {
//     // Act
//     let presses = part2(path);

//     // Assert
//     assert_eq!(presses, expected_presses);
// }