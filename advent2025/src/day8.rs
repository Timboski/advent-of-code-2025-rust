use crate::utils::read_file_lines;
// use rstest::rstest;

#[allow(dead_code)]
pub fn main() {
    // let path = "/workspaces/advent-of-code-2025-rust/day8-example.txt";
    let path = "/workspaces/advent-of-code-2025-rust/day8-input.txt";

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

    jbs.sort_by_key(|k| k.distance_squared);
    for jb in jbs {println!("{:?} {:?} {:?}",jb, boxes[jb.start], boxes[jb.end])}
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
