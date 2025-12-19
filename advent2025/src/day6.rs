use crate::utils::read_file_lines;
use rstest::rstest;

#[allow(dead_code)]
pub fn main() {
    // let path = "/workspaces/advent-of-code-2025-rust/day6-example.txt";
    let path = "/workspaces/advent-of-code-2025-rust/day6-input.txt";

    println!("Part 1");
    let total1 = part1(path);
    
    println!();
    println!("Part 2");
    let total2 = part2(path);

    println!();
    println!("Grand Total (Part1): {}", total1);
    println!("Grand Total (Part2): {}", total2);
}

fn part1(path: &str) -> u64 {
    let problems = decode_puzzle_input(path);

    solve(problems)
}

fn part2(path: &str) -> u64 {
    let problems = decode_puzzle_input(path);

    // Manipulate the strings
    let mut adjusted_problems: Vec<(char, Vec<String>)> = Vec::new();
    for problem in problems {
        let operation = problem.0;
        let arguments = decode_from_cephalopod(problem.1);
        adjusted_problems.push((operation, arguments));
    }

    solve(adjusted_problems)
}

fn decode_from_cephalopod(arguments: Vec<String>) -> Vec<String> {
    let longest_input = arguments.iter().map(|a| a.len()).max().unwrap();
    let mut output: Vec<String> = vec![String::new(); longest_input];
    for i in (0..longest_input).rev() {
        for arg in arguments.iter().filter(|a| a.len() > i) {
            output[i].push(arg.chars().skip(i).next().unwrap());
        }
    }

    output
}

fn solve(problems: Vec<(char, Vec<String>)>) -> u64 {
    let mut total = 0;
    for problem in problems {
        total += solver(problem)
    }

    total
}

fn decode_puzzle_input(path: &str) -> Vec<(char, Vec<String>)> {
    let lines = read_file_lines(path).unwrap();

    let matrix: Vec<Vec<String>> = split_on_all_space(&lines);

    let num_problems = matrix[0].len();
    println!("Number of problems: {}", num_problems);

    let mut problems: Vec<(char, Vec<String>)> = Vec::new();
    for mut element in matrix {
        let operator = element.pop().unwrap().trim().chars().next().unwrap();
        problems.push((operator, element));
    }

    problems
}

/// Prompt: in rust take a vector of string and split them at points where all string are a space character. Leave the remaining whitespace in the sub strings. output should be a vector of vector of string
/// 
/// Split a group of strings simultaneously at character positions
/// where *every* string has a space (' ') character.
/// The split removes those shared space columns, but keeps all other whitespace.
///
/// Returns a Vec of segments; each segment is a Vec<String> with the
/// same length as the input, containing the piece for each original string.
///
/// Example:
/// input: ["ab  cd e", "xy  z  t"]
/// shared space columns are at indices 2 and 5 (0-based char indices, both strings have ' ' there)
/// output segments:
///   [ ["ab", "xy"], ["", ""], ["cd", "z"], ["e", "t"] ]
pub fn split_on_all_space(input: &[String]) -> Vec<Vec<String>> {
    if input.is_empty() {
        return Vec::new();
    }

    // Precompute (char_index -> byte_offset) maps and char vectors for each string
    // to safely go between char index and slice boundaries.
    let per_str_char_indices: Vec<Vec<(usize, char)>> = input
        .iter()
        .map(|s| s.char_indices().collect::<Vec<_>>())
        .collect();

    // Find the maximum number of chars among input strings.
    // We will only consider positions that exist in all strings (shared indices).
    let min_char_len = per_str_char_indices
        .iter()
        .map(|v| v.len())
        .min()
        .unwrap_or(0);

    // Identify split positions: char index `i` where every string has ' ' at that char.
    let mut split_positions_char: Vec<usize> = Vec::new();
    for i in 0..min_char_len {
        let all_space = per_str_char_indices
            .iter()
            .all(|v| v[i].1 == ' ');
        if all_space {
            split_positions_char.push(i);
        }
    }

    // Convert char indices to per-string byte offsets for slicing.
    // We'll build segments between these split positions. The separator space columns are excluded.
    // Segment boundaries: previous_split+1 .. current_split (in char-index space)
    // With an initial segment from 0 .. first_split, and a trailing segment after last_split .. end.
    //
    // For slicing, we need byte offsets for start and end char indices for each string.
    // Helper to convert char index -> byte offset (start of that char), or end byte offset when using end index.
    let mut segments: Vec<Vec<String>> = Vec::new();

    // Prepare an iterator of segment ranges in char-index space.
    // Example with splits at [s0, s1, s2]:
    // ranges: [0 .. s0), (s0+1 .. s1), (s1+1 .. s2), (s2+1 .. len)
    // Note: (a .. b) uses exclusive end b.
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    let mut prev_end = 0usize;

    for &split_at in &split_positions_char {
        // Current segment is prev_end .. split_at (exclusive)
        ranges.push((prev_end, split_at));
        // Next segment starts after the separator column (skip the space at split_at)
        prev_end = split_at + 1;
    }
    // Trailing segment
    ranges.push((prev_end, min_char_len));

    // For each range, produce a piece for each string using byte offsets.
    for (start_char, end_char) in ranges {
        let mut segment_pieces: Vec<String> = Vec::with_capacity(input.len());
        for (s_idx, s) in input.iter().enumerate() {
            let ci = &per_str_char_indices[s_idx];

            // If start == end, this segment piece is empty string.
            if start_char == end_char {
                segment_pieces.push(String::new());
                continue;
            }

            // Get byte start: byte offset of char at start_char
            let byte_start = ci[start_char].0;

            // Get byte end: if end_char == len, end is s.len(),
            // else it's byte offset of char at end_char
            let byte_end = if end_char < ci.len() {
                ci[end_char].0
            } else {
                s.len()
            };

            // Safety: byte_start and byte_end are on char boundaries by construction.
            let piece = s[byte_start..byte_end].to_string();
            segment_pieces.push(piece);
        }
        segments.push(segment_pieces);
    }

    segments
}


fn solver(problem: (char, Vec<String>)) -> u64 {
    let mut answer = match problem.0 {            
            '*' => 1,
            '+' => 0,
            _ => panic!(),
    };
    for arg_idx in 0..problem.1.len() {
        let argument: u64 = (problem.1[arg_idx]).trim().parse().unwrap();
        if arg_idx > 0 {print!("{}",problem.0)}
        print!("{}",argument);
        answer = match problem.0 {
            '*' => answer * argument,
            '+' => answer + argument,
            _ => panic!(),
        }
    }

    println!("={}", answer);
    answer
}


#[rstest]
#[case("/workspaces/advent-of-code-2025-rust/day6-example.txt", 4277556)]
#[case("/workspaces/advent-of-code-2025-rust/day6-input.txt", 5346286649122)]
fn test_part1_answers(
    #[case] path: &str,
    #[case] expected_total: u64
)
{
    // Act
    let total = part1(path);

    // Assert
    assert_eq!(total, expected_total);
}

#[rstest]
#[case("/workspaces/advent-of-code-2025-rust/day6-example.txt", 3263827)]
#[case("/workspaces/advent-of-code-2025-rust/day6-input.txt", 10389131401929)]
fn test_part2_answers(
    #[case] path: &str,
    #[case] expected_total: u64
)
{
    // Act
    let total = part2(path);

    // Assert
    assert_eq!(total, expected_total);
}