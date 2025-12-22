use std::collections::HashMap;
use crate::utils::read_file_lines;
use rstest::rstest;

#[allow(dead_code)]
pub fn main() {
    // let path = "/workspaces/advent-of-code-2025-rust/day9-example.txt";
    let path = "/workspaces/advent-of-code-2025-rust/day9-input.txt";

    let area1 = part_1(path);
    let area2 = part_2(path);

    println!("Biggest rectangle overall: {}", area1);
    println!("Biggest rectangle just green and red: {}", area2);
}

fn part_1(path: &str) -> u64 {
    let points = read_points_from_file(path);
    let areas = compute_rectangles(points);
    areas.first().unwrap().0
}

fn compute_rectangles(points: Vec<(u32, u32)>) -> Vec<(u64, u32, u32, u32, u32)> {
    let mut areas = Vec::new();
    for point1 in &points {
        for point2 in &points {
            let area = compute_side_length(point1.0, point2.0) * compute_side_length(point1.1, point2.1);
            areas.push((area,point1.0, point2.0, point1.1, point2.1));
        }
    }

    areas.sort();
    areas.reverse();

    areas
}

fn read_points_from_file(path: &str) -> Vec<(u32, u32)> {
    let lines = read_file_lines(path).unwrap();
    let points: Vec<(u32, u32)> = lines.iter()
        .map(
            |l| {
                let coords = l.split_once(",").unwrap();
                (coords.0.parse().unwrap(), coords.1.parse().unwrap())
            }                
        )
        .collect();
    points
}

fn compute_side_length(point1: u32, point2: u32) -> u64 {
    point1.abs_diff(point2) as u64 + 1
}

fn part_2(path: &str) -> u64 {
    let points = read_points_from_file(path);

    // Visualise the floor area
    let mut x_values: Vec<u32> = points.iter().map(|p| p.0).collect();
    x_values.sort();
    x_values.dedup();
    let x_map: HashMap<&u32, usize> = x_values.iter()
        .enumerate()
        .map(|(i, val)| (val, i))
        .collect();

    let mut y_values: Vec<u32> = points.iter().map(|p| p.1).collect();
    y_values.sort();
    y_values.dedup();
    let y_map: HashMap<&u32, usize> = y_values.iter()
        .enumerate()
        .map(|(i, val)| (val, i))
        .collect();

    let mut matrix = vec![vec!['.'; x_map.len()]; y_map.len()];
    let mut previous_point = points.last().unwrap();
    for point in &points {
        let prev_x = x_map[&previous_point.0];
        let prev_y = y_map[&previous_point.1];
        let point_x = x_map[&point.0];
        let point_y = y_map[&point.1];
        matrix[point_y][point_x] = '#';

        // Draw in the greens
        for x in get_range(prev_x, point_x) {
            for y in get_range(prev_y, point_y) {
                if matrix[y][x] == '.' { matrix[y][x] = 'X';}
            }
        }

        previous_point = &point;
    }

    // Fill in the shape
    // If we have a Red-Greens-Red pattern (e.g. #X# or #XX#) then we cannot know
    // (just from that line) if the following tiles are inside or outside.
    // Assumption - there is only 1 such (#XXX#) pattern on each row, so we can stop
    // when we hit it. By scanning from both side we will get to all tiles.
    // Note: Red tiles are always part of the #X# pattern so can just stop if we hit Red
    for mut row in &mut matrix {
        // Run left to right
        fill_inside_tiles(&mut row);
        row.reverse();
        fill_inside_tiles(&mut row);
        row.reverse();
    }

    // Plot the tiles
    for y in &matrix {
        for x in y {
            print!("{}", x);
        }
        println!();
    }

    // Find the rectangles to test
    let rectangles = compute_rectangles(points);
    for rectangle in rectangles {
        // Map to the normalised coordinates
        let x1 = x_map[&rectangle.1];
        let x2 = x_map[&rectangle.2];
        let y1 = y_map[&rectangle.3];
        let y2 = y_map[&rectangle.4];

        // Test each of the lines
        // Assumption: If all of the lines are inside the shape, the whole rectangle must be inside the shape.
        // Note: This holds as long as the shape contains no holes (which it doesn't)
        if is_line_inside_shape(x1, y1, x1, y2, &matrix) && 
            is_line_inside_shape(x2, y1, x2, y2, &matrix) && 
            is_line_inside_shape(x1, y1, x2, y1, &matrix) && 
            is_line_inside_shape(x1, y2, x2, y2, &matrix) { 
            // We have found the result.
            return rectangle.0
        }
    }

    0
}

fn is_line_inside_shape(x1: usize, y1: usize, x2: usize, y2: usize, matrix: &Vec<Vec<char>>) -> bool {
    for x in get_range(x1,x2) {
        for y in get_range(y1, y2) {
            if matrix[y][x] == '.' { return false; }
        }
    }
    
    true
}

fn fill_inside_tiles(row: &mut Vec<char>) {
    let mut inside = false;
    for tile in row.iter_mut() {
        match *tile {
            '.' => if inside { *tile = 'x'; }
            'X' => { inside = !inside; }
            '#'| 'x' => { break; }
            _ => panic!()
        }
    }
}

fn get_range(previous_point: usize, point: usize) -> std::ops::RangeInclusive<usize> {
    if previous_point < point { return previous_point..=point; }
    point..=previous_point
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

#[rstest]
#[case("/workspaces/advent-of-code-2025-rust/day9-example.txt", 24)]
#[case("/workspaces/advent-of-code-2025-rust/day9-input.txt", 1550760868)]
fn test_part2_answers(
    #[case] path: &str,
    #[case] expected_area: u64
)
{
    // Act
    let max_area = part_2(path);

    // Assert
    assert_eq!(max_area, expected_area);
}