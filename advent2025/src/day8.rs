use crate::utils::read_file_lines;
// use rstest::rstest;

#[allow(dead_code)]
pub fn main() {
    let path = "/workspaces/advent-of-code-2025-rust/day8-example.txt";
    // let path = "/workspaces/advent-of-code-2025-rust/day8-input.txt";

    let boxes: Vec<Point3D> = read_file_lines(path)
        .unwrap()
        .iter()
        .map(|l| Point3D::new(l))
        .collect();

    let mut jbs = Vec::new();
    for (box_index, b) in boxes.iter().enumerate() {        
        let (index, distance) = boxes.iter()
            .enumerate()
            .filter(|(_, point)| *point != b)
            .map(|(index, point)| (index, b.distance_squared(point)))
            .min_by_key(|k| k.1)
            .unwrap();
        jbs.push(Connection {start: box_index, end: index, distance_squared: distance});
    }

    // Find duplicates
    let mut duplicates = Vec::new();
    for jb in &jbs {
        let other = &jbs[jb.end];
        assert!(jb.end == other.start);
        if jb.start == other.end {
            // Duplicate entry
            //print!("Duplicate found {:?} {:?}", jb, other);
            if duplicates.contains(&(jb.start)) {
                //print!(" - already in list");
            }
            else {                
                duplicates.push(jb.end);
            }
            //println!();
        }
    }
    // println!("Duplicates found: {:?}", duplicates);

    // Remove duplicates
    // We must do this starting with the highest index so as not to change the duplicate indices.
    duplicates.sort();
    duplicates.reverse();
    for dup in duplicates {
        // println!("Removing: {}", dup);
        jbs.remove(dup);
    }

    jbs.sort_by_key(|k| k.distance_squared);
    for jb in &jbs {println!("{:?} {:?} {:?}",jb, boxes[jb.start], boxes[jb.end])}

    // Make groups
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for jb in jbs.iter().take(10) {
        println!();
        for (index, group) in groups.iter().enumerate() {println!("{}: {:?}", index, group)};
        let start_group = find_group(&groups, jb.start);
        let end_group = find_group(&groups, jb.end);
        print!("Connection {}-{}: {:?} {:?} : ", jb.start, jb.end, start_group, end_group);

        match start_group {
            None => match end_group {
                None => {
                    println!("New Group Needed");
                    groups.push(vec![jb.start, jb.end]);
                }
                Some(end) => {
                    println!("Add start to group {}", end);
                    groups[end].push(jb.start);
                }
            }
            Some(start) => match end_group {
                None => {
                    println!("Add end to group {}", start);
                    panic!("Not implemented");
                }
                Some(end) => {
                    println!("Combine Groups {} and {}", start, end);
                    panic!("Not implemented");
                }
            }
        }
    }

}

fn find_group(groups: &Vec<Vec<usize>>, index: usize) -> Option<usize> {
    print!("Find {} in {:?} ", index, groups);
    let start_group = groups
        .iter()
        .enumerate()
        .filter(|(_, members)| members.contains(&index))
        .map(|(index, _)| index)
        .next();

    println!("-> {:?}", start_group);
    start_group
}

#[derive(PartialEq, Eq, Debug)]
struct Point3D {
    x: u32,
    y: u32,
    z: u32,
}

impl Point3D {
    fn new(line: &String) -> Self {
        let coordinates: Vec<u32> = line.split(',')
            .map(|c| c.parse().unwrap())
            .collect();
        Point3D { x: coordinates[0], y: coordinates[1], z: coordinates[2] }
    }

    /// No need to do the square root as we are just using for comparison
    pub fn distance_squared(&self, other: &Point3D) -> u64 {
        get_squared_distance(self.x, other.x) + get_squared_distance(self.y, other.y) + get_squared_distance(self.z, other.z)
    }
}

fn get_squared_distance(pos: u32, other_pos: u32) -> u64 {
    let dist = pos as i64 - other_pos as i64;
    (dist * dist) as u64
}

#[derive(Debug)]
struct Connection {
    start: usize,
    end: usize,
    distance_squared: u64,
}
