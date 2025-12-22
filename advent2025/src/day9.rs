use crate::utils::read_file_lines;
use rstest::rstest;

#[allow(dead_code)]
pub fn main() {
    // let path = "/workspaces/advent-of-code-2025-rust/day9-example.txt";
    let path = "/workspaces/advent-of-code-2025-rust/day9-input.txt";

    let max_area = part_1(path);

    println!("Biggest rectangle: {}", max_area);
}

fn part_1(path: &str) -> u64 {
    let lines = read_file_lines(path).unwrap();
    let points: Vec<(u32, u32)> = lines.iter()
        .map(
            |l| {
                let coords = l.split_once(",").unwrap();
                (coords.0.parse().unwrap(), coords.1.parse().unwrap())
            }                
        )
        .collect();

    let mut max_area = 0;
    for point1 in &points {
        for point2 in &points {
            let area = compute_side_length(point1.0, point2.0) * compute_side_length(point1.1, point2.1);
            println!("Compare: {:?} -> {:?} = {}", point1, point2, area);
            max_area = max_area.max(area);
        }
    }
    max_area
}

fn compute_side_length(point1: u32, point2: u32) -> u64 {
    point1.abs_diff(point2) as u64 + 1
}


#[rstest]
#[case("/workspaces/advent-of-code-2025-rust/day9-example.txt", 50)]
#[case("/workspaces/advent-of-code-2025-rust/day9-input.txt", 4748985168)]
fn test_part1_answers(
    #[case] path: &str,
    #[case] expected_area: u64
)
{
    // Act
    let max_area = part_1(path);

    // Assert
    assert_eq!(max_area, expected_area);
}