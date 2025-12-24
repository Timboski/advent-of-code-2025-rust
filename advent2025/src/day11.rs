use rstest::rstest;

use crate::utils::read_file_lines;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
    // let path = "/workspaces/advent-of-code-2025-rust/day11-example.txt";
    // let path = "/workspaces/advent-of-code-2025-rust/day11-example2.txt";
    let path = "/workspaces/advent-of-code-2025-rust/day11-input.txt";

    let num_paths = part_1(path);    
    
    println!();
    println!("Paths found: {}", num_paths);

    return;
}

fn part_1(path: &str) -> u64 {
    let (connection, order) = compute_connections_and_topological_order(path);
    let scores = find_number_of_paths(connection, order);
    scores["you"]
}


/// Compute the number of paths to the end for each device
fn find_number_of_paths(connection: HashMap<String, Vec<String>>, order: Vec<String>) -> HashMap<String, u64> {
    // Workthrough the devices in topological order assigning a score to each device.
    // The score is the number of routes to the end from that device.
    // This can be computed by summing the number of routes on each of its outgoing connections.
    let mut scores = HashMap::new();
    for device in order {
        println!("Scoring {}", device);
        let links = &connection.get(&device);
        let score = match links {
            Some(l) => { l.iter().map(|i| scores[i]).sum() }
            None => 1u64,
        };
        scores.insert(device, score);
    }

    println!("Scores: {:?}", scores);
    scores
}

fn compute_connections_and_topological_order(path: &str) -> (HashMap<String, Vec<String>>, Vec<String>) {
    // Build a list of the outgoing connections from each device
    let connection: HashMap<String, Vec<String>> = read_file_lines(path).unwrap().iter().map(|l| get_device_details(l)).collect();
    println!("Devices: {:?}", connection);

    // Count the number of incoming connections into each device.
    // NOTE: Populate from keys so that devices with no connections appear.
    let mut indegree: HashMap<String, u32> = connection.keys().map(|k| (k.clone(), 0)).collect();
    for links in connection.values() {
        for link in links {
            *indegree.entry(link.clone()).or_insert(0) += 1;
        }
    }
    println!("Indegrees: {:?}", indegree);

    // Create a topological ordering using Khan's algorithm
    let mut order: Vec<String> = Vec::new();
    loop {
        // Get an item with 0 in links            
        let next = match indegree.iter().find(|(_, v)| **v == 0) {
            Some(n) => n.0.clone(),
            None => { break }
        };
        indegree.remove(&next);
        order.push(next.clone());
        print!("Popped: {}", next);

        // Remove the inlinks from this device to other devices
        let links = &connection.get(&next);
        match links {
            Some(l) => {                
                println!(" - links to {:?}", l);
                for link in *l {
                    assert!(!order.contains(link)); // Can't reference earler devices
                    if indegree.contains_key(link) {                
                        *indegree.entry(link.clone()).or_insert(0) -= 1;
                    }
                }
            }
            None => { println!(" - end point")}
        }
    }
    assert!(indegree.len() == 0);
    // Check all elements used up

    // Work backwards through the list assigning number of routes to the end for each device.
    // Number of routes is the sum of the number of routes on each of its connections.
    order.reverse();
    println!("Topological order of devices (reversed) {:?}", order);

    (connection, order)
}

fn get_device_details(devce_definition: &str) -> (String, Vec<String>) {
    let (id, connections) = devce_definition.split_once(":").unwrap();
    ( id.to_string(), connections.split_whitespace().map(|s| s.to_string()).collect())
}


#[rstest]
#[case("/workspaces/advent-of-code-2025-rust/day11-example.txt", 5)]
#[case("/workspaces/advent-of-code-2025-rust/day11-input.txt", 500)]
fn test_part1_answers(
    #[case] path: &str,
    #[case] expected_paths: u64
)
{
    // Act
    let max_area = part_1(path);

    // Assert
    assert_eq!(max_area, expected_paths);
}