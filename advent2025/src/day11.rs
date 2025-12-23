use crate::utils::read_file_lines;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
    // let path = "/workspaces/advent-of-code-2025-rust/day11-example.txt";
    let path = "/workspaces/advent-of-code-2025-rust/day11-input.txt";

    let devices: HashMap<String, Vec<String>> = read_file_lines(path).unwrap().iter().map(|l| get_device_details(l)).collect();

    let mut paths_found = 0;
    let mut tracers = vec!["you".to_string()];
    loop {
        let tracer = match tracers.pop() {
            Some(t) => t,
            None => { break; },
        };

        for connection in devices.get(&tracer).unwrap() {
            if connection == "out" {
                paths_found += 1;
            } else {
                tracers.push(connection.clone());
            }
        }
    }

    println!("Paths found: {}", paths_found);
}

fn get_device_details(devce_definition: &str) -> (String, Vec<String>) {
    let (id, connections) = devce_definition.split_once(":").unwrap();
    ( id.to_string(), connections.split_whitespace().map(|s| s.to_string()).collect())
}