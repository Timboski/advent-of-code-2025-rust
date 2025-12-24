use crate::utils::read_file_lines;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
    // let path = "/workspaces/advent-of-code-2025-rust/day11-example.txt";
    // let path = "/workspaces/advent-of-code-2025-rust/day11-example2.txt";
    let path = "/workspaces/advent-of-code-2025-rust/day11-input.txt";

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

    println!("Remaining elements: {:?}", indegree);


    return;


    let mut paths_found = 0;
    let mut valid_paths_found = 0;
    let mut tracers = vec![("svr".to_string(), false, false)];
    loop {
        let (tracer, dac_seen, fft_seen) = match tracers.pop() {
            Some(t) => t,
            None => { break; },
        };

        for connection in connection.get(&tracer).unwrap() {
            if connection == "out" {
                paths_found += 1;
                if dac_seen && fft_seen { valid_paths_found += 1 }
            } else {
                let now_dac_seen = dac_seen || connection == "dac";
                let now_fft_seen = fft_seen || connection == "fft";
                tracers.push((connection.clone(), now_dac_seen, now_fft_seen));
            }
        }
    }

    println!("Paths found: {}", paths_found);
    println!("Valid Paths found: {}", valid_paths_found);
}

fn get_device_details(devce_definition: &str) -> (String, Vec<String>) {
    let (id, connections) = devce_definition.split_once(":").unwrap();
    ( id.to_string(), connections.split_whitespace().map(|s| s.to_string()).collect())
}