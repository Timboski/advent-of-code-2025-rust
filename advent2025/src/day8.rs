use crate::utils::read_file_lines;
// use rstest::rstest;

#[allow(dead_code)]
pub fn main() {
    // let path = "/workspaces/advent-of-code-2025-rust/day8-example.txt";
    let path = "/workspaces/advent-of-code-2025-rust/day8-input.txt";

    let max_number_of_connections= 1000;

    let answer = part1(path, max_number_of_connections);

    println!("Size of top 3 groups: {}", answer);
}

fn part1(path: &str, max_number_of_connections: usize) -> usize {
    // Read the file
    let boxes: Vec<Point3D> = read_file_lines(path)
        .unwrap()
        .iter()
        .map(|l| Point3D::new(l))
        .collect();

    // Find all possible connections
    let mut jbs = Vec::new();
    for (box_index, b) in boxes.iter().enumerate() {        
        for (index, distance) in boxes.iter()
            .enumerate()
            .filter(|(_, point)| *point != b)
            .map(|(index, point)| (index, b.distance_squared(point)))
        {
            jbs.push(PotentialConnection {start: box_index, end: index, distance_squared: distance});
        }
    }

    // Sort connections by distance
    jbs.sort_by_key(|k| k.distance_squared);
    println!("Potential connections: {}", jbs.len());
    for jb in jbs.iter().take(20) {println!("{:?} {:?} {:?}",jb, boxes[jb.start], boxes[jb.end])}

    // Make groups
    let mut connections: Vec<Connection> = Vec::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for jb in &jbs {
        println!("Num connections made: {}", connections.len());
        for (index, group) in groups.iter().enumerate() {println!("{}: {:?}", index, group)};
        if connections.len() >= max_number_of_connections { break }
        println!();
        if connections.iter().any(|c| c.isSame(&jb)) {             
            println!("Connection {}-{} already made", jb.start, jb.end);
            continue;
        }

        let start_group = find_group(&groups, jb.start);
        let end_group = find_group(&groups, jb.end);
        print!("Connection {}-{}: {:?} {:?} : ", jb.start, jb.end, start_group, end_group);
        connections.push(Connection { start: jb.start, end: jb.end });

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
                    groups[start].push(jb.end);
                }
                Some(end) => {
                    if start == end {
                        println!("Already in same group");
                    } else {
                        println!("Combine Groups {} and {}", start, end);

                        let mut end_elements = Vec::new();
                        end_elements.append(&mut groups[end]);
                        groups[start].append(&mut end_elements);
                        groups.remove(end);
                    }
                }
            }
        }
    }

    println!();
    println!("FINISHED");
    groups.sort_by_key(|g| g.len());
    groups.reverse();
    for (index, group) in groups.iter().enumerate() {println!("{}: {:?}", index, group)};
    let answer: usize = groups.iter().take(3).map(|g| g.len()).product();
    answer
}

fn find_group(groups: &Vec<Vec<usize>>, index: usize) -> Option<usize> {
    // print!("Find {} in {:?} ", index, groups);
    let start_group = groups
        .iter()
        .enumerate()
        .filter(|(_, members)| members.contains(&index))
        .map(|(index, _)| index)
        .next();

    // println!("-> {:?}", start_group);
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
struct PotentialConnection {
    start: usize,
    end: usize,
    distance_squared: u64,
}

#[derive(Debug)]
struct Connection {
    start: usize,
    end: usize
}

impl Connection {
    fn isSame(&self, connection: &PotentialConnection) -> bool {
        if self.start == connection.start && self.end == connection.end {
            return true;
        }

        if self.start == connection.end && self.end == connection.start {
            return true;
        }

        return false;
    }
}