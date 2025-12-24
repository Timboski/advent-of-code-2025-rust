// Prompt:
// in rust read a matrix from a file. Each line contains . for an empty space and @ for a full space. Find the number of full spaces which have fewer than 4 adjacent spaces filled. count adjacent as the 8 surrounding spaces.

use rstest::rstest;
use std::fs;
use std::process;

#[allow(dead_code)]
pub fn main() {
    // Expect exactly one argument: the file path
    // let path = "/workspaces/advent-of-code-2025-rust/day4-example.txt";
    let path = "/workspaces/advent-of-code-2025-rust/day4-input.txt";

    let count1 = part1(path);
    let count2 = part2(path);

    println!("Single unloading step - {}", count1);
    println!("All possible steps - {}", count2);
}

fn part1(path: &str) -> usize {
    let grid = load_grid_from_file(path);

    let count = count_and_clear_fewer_than_4_neighbors(&grid).0;
    count
}

fn part2(path: &str) -> usize {
    let mut grid = load_grid_from_file(path);

    let mut count = 0;
    loop {
        let frame = count_and_clear_fewer_than_4_neighbors(&grid);
        count += frame.0;
        grid = frame.1;
        if frame.0 == 0 {
            break;
        }
    }

    count
}

fn load_grid_from_file(path: &str) -> Vec<Vec<bool>> {
    // Load the file contents
    let contents = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", path, e);
            process::exit(1);
        }
    };

    // Parse into a boolean grid: true = filled (@), false = empty (.)
    let grid = match parse_grid(&contents) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Input error: {}", e);
            process::exit(1);
        }
    };
    grid
}

/// Parse the input text into a rectangular grid.
/// Returns an error if lines are empty, have inconsistent width,
/// or contain invalid characters (not '.' or '@').
fn parse_grid(input: &str) -> Result<Vec<Vec<bool>>, String> {
    let mut grid: Vec<Vec<bool>> = Vec::new();
    let mut width: Option<usize> = None;

    for (line_idx, raw_line) in input.lines().enumerate() {
        // Allow empty lines to be ignored; but if all lines are empty, it's invalid.
        let line = raw_line.trim_end(); // Keep leading spaces if any; only trim trailing newline
        if line.is_empty() {
            continue;
        }

        let row: Result<Vec<bool>, String> = line
            .chars()
            .map(|c| match c {
                '.' => Ok(false),
                '@' => Ok(true),
                other => Err(format!(
                    "Invalid character '{}' at line {}. Only '.' and '@' are allowed.",
                    other,
                    line_idx + 1
                )),
            })
            .collect();

        let row = row?;

        match width {
            Some(w) => {
                if row.len() != w {
                    return Err(format!(
                        "Inconsistent row width at line {}: expected {}, found {}",
                        line_idx + 1,
                        w,
                        row.len()
                    ));
                }
            }
            None => {
                if row.is_empty() {
                    return Err(format!("Line {} is empty.", line_idx + 1));
                }
                width = Some(row.len());
            }
        }

        grid.push(row);
    }

    if grid.is_empty() {
        return Err("No non-empty lines found in the file.".to_string());
    }

    Ok(grid)
}

/// Count how many filled cells have fewer than 4 filled neighbors (8-directional),
/// and return a new grid where those counted cells are set to false.
///
/// Returns: (count, new_grid)
#[rustfmt::skip] // Allow the directions to be displayed pictorially
pub fn count_and_clear_fewer_than_4_neighbors(grid: &[Vec<bool>]) -> (usize, Vec<Vec<bool>>) {
    // Handle empty cases safely
    if grid.is_empty() || grid[0].is_empty() {
        return (0, grid.to_owned());
    }

    let h = grid.len();
    let w = grid[0].len();

    let directions: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    // Start with a copy of the input grid that we will modify
    let mut new_grid = grid.to_owned();
    let mut count = 0usize;

    for r in 0..h {
        for c in 0..w {
            if !grid[r][c] {
                continue; // Only consider filled cells
            }

            // Count filled neighbors (8-directional)
            let mut neighbors_filled = 0;
            for (dr, dc) in directions.iter() {
                let nr = r as isize + dr;
                let nc = c as isize + dc;

                if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
                    if grid[nr as usize][nc as usize] {
                        neighbors_filled += 1;
                    }
                }
            }

            if neighbors_filled < 4 {
                count += 1;
                new_grid[r][c] = false; // Clear this cell in the output grid
            }
        }
    }

    (count, new_grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_shape_example() {
        // .@.
        // @@@
        // .@.
        // Center has exactly 4 neighbors; arms have fewer than 4.
        let input = ".@.\n@@@\n.@.\n";
        let grid = parse_grid(input).unwrap();
        let count = count_and_clear_fewer_than_4_neighbors(&grid).0;
        assert_eq!(count, 4);
    }

    #[test]
    fn isolated_points() {
        // @..
        // ...
        // ..@
        let input = "@..\n...\n..@\n";
        let grid = parse_grid(input).unwrap();
        let count = count_and_clear_fewer_than_4_neighbors(&grid).0;
        // Both filled cells have 0 neighbors, which is < 4.
        assert_eq!(count, 2);
    }

    #[test]
    fn dense_block() {
        // @@
        // @@
        // Each interior (all of them) has 3 neighbors at edges? Let's check:
        // In a 2x2 block, each cell has 3 neighbors within 8-neighborhood.
        // So all 4 cells have neighbors_filled = 3, thus counted.
        let input = "@@\n@@\n";
        let grid = parse_grid(input).unwrap();
        let count = count_and_clear_fewer_than_4_neighbors(&grid).0;
        assert_eq!(count, 4);
    }
}

#[rstest]
#[case("/workspaces/advent-of-code-2025-rust/day4-example.txt", 13)]
#[case("/workspaces/advent-of-code-2025-rust/day4-input.txt", 1489)]
fn test_part1_answers(#[case] path: &str, #[case] expected_rolls: usize) {
    // Act
    let rolls = part1(path);

    // Assert
    assert_eq!(rolls, expected_rolls);
}

#[rstest]
#[case("/workspaces/advent-of-code-2025-rust/day4-example.txt", 43)]
#[case("/workspaces/advent-of-code-2025-rust/day4-input.txt", 8890)]
fn test_part2_answers(#[case] path: &str, #[case] expected_rolls: usize) {
    // Act
    let rolls = part2(path);

    // Assert
    assert_eq!(rolls, expected_rolls);
}
