use rstest::rstest;

pub fn main() {
    let path = "/workspaces/advent-of-code-2025-rust/day2-input.txt";

    let sum = find_password(path);

    println!("Password: {}", sum);
}

fn find_password(path: &str) -> u128 {
    let ranges = read_tuple_line(path).unwrap();

    let mut sum: u128 = 0;
    for range in ranges {
        let invalid_ids = find_invalid_ids(range.start, range.end);
        println!("Start {}, End {}, Invalid IDs: {:?}", range.start, range.end, invalid_ids);
        let total: u128 = invalid_ids.iter().sum();
        sum += total;
    }

    sum
}

fn find_invalid_ids(range_start: u128, range_end: u128) -> Vec<u128> {
    let start = find_first_invalid_id_sequence(range_start);
    let mut invalid_ids: Vec<u128> = Vec::new();
    
    let mut current = start;
    loop {
        let invalid_id = duplicate_block(current);
        if invalid_id > range_end { break; }
        invalid_ids.push(invalid_id);
        current += 1;
    }
    
    invalid_ids
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: u128,
    pub end: u128,
}


use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

pub fn read_tuple_line<P: AsRef<Path>>(path: P) -> Result<Vec<Range>, ParseTuplesError> {
    let file = File::open(path.as_ref()).map_err(ParseTuplesError::Io)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    let bytes = reader.read_line(&mut line).map_err(ParseTuplesError::Io)?;
    if bytes == 0 {
        return Ok(Vec::new()); // empty file -> empty list
    }

    let line = line.trim();
    if line.is_empty() {
        return Ok(Vec::new());
    }

    parse_tuple_list(line)
}

fn parse_tuple_list(s: &str) -> Result<Vec<Range>, ParseTuplesError> {
    let mut out = Vec::new();

    for (idx, item) in s.split(',').enumerate() {
        let item = item.trim();
        if item.is_empty() {
            continue;
        }

        let mut parts = item.splitn(2, '-').map(str::trim);
        let left = parts.next().ok_or_else(|| ParseTuplesError::MalformedItem {
            item_index: idx,
            item: item.to_string(),
            reason: "missing '-'".to_string(),
        })?;
        let right = parts.next().ok_or_else(|| ParseTuplesError::MalformedItem {
            item_index: idx,
            item: item.to_string(),
            reason: "missing right side after '-'".to_string(),
        })?;

        let start = left.parse::<u128>().map_err(|e| ParseTuplesError::ParseNumber {
            item_index: idx,
            side: "left",
            text: left.to_string(),
            source: e,
        })?;
        let end = right.parse::<u128>().map_err(|e| ParseTuplesError::ParseNumber {
            item_index: idx,
            side: "right",
            text: right.to_string(),
            source: e,
        })?;

        out.push(Range {
            start,
            end,
        });
    }

    Ok(out)
}

#[derive(Debug)]
pub enum ParseTuplesError {
    Io(io::Error),
    MalformedItem { item_index: usize, item: String, reason: String },
    ParseNumber {
        item_index: usize,
        side: &'static str, // "left" or "right"
        text: String,
        source: std::num::ParseIntError,
    },
}

impl std::fmt::Display for ParseTuplesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseTuplesError::Io(e) => write!(f, "I/O error: {}", e),
            ParseTuplesError::MalformedItem { item_index, item, reason } => {
                write!(f, "Malformed item at index {} ('{}'): {}", item_index, item, reason)
            }
            ParseTuplesError::ParseNumber { item_index, side, text, source } => {
                write!(f, "Failed to parse {} number at item {} ('{}'): {}", side, item_index, text, source)
            }
        }
    }
}
impl std::error::Error for ParseTuplesError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
                       ParseTuplesError::Io(e) => Some(e),
            ParseTuplesError::ParseNumber { source, .. } => Some(source),
            _ => None,
        }
    }
}


pub fn find_first_invalid_id_sequence(range_start: u128) -> u128 {
    let s = range_start.to_string();
    let digits = s.len();
    
    let first_sequence: u128 = if digits % 2 == 1 {
        // Odd number of digits. Round up to the smallest number with even digits (and chop in half)
        // This is the same as 1 followed by a zero for each of half the original number of digits (rounded down).
        let zeros_count = digits / 2;
        let even_start = format!("1{}", "0".repeat(zeros_count));
        even_start.parse::<u128>().unwrap()
    } else {
        // Just grab the first half of the digits using string manipulation.
        let start_as_string = range_start.to_string();
        let half_size = start_as_string.len() / 2;
        let left_half = &start_as_string[..half_size];
        left_half.parse::<u128>().unwrap()
    };

    // If the range_start is not an invalid id, the duplicated sequence will be below the range.
    // Incrementing by one will bring it into (or over) the range.
    if duplicate_block(first_sequence) < range_start { return first_sequence + 1; }

    // The range start is an invalid id, so ensure this is included.
    return first_sequence;
}

fn duplicate_block(n: u128) -> u128 {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() * 2);
    out.push_str(&s);
    out.push_str(&s);
    out.parse::<u128>().ok().unwrap()
}


#[test]
pub fn test_find_first_invalid_id_sequence() {
    // 9 has 1 digit (odd) -> round to 10 (even 2 digits) -> first invalid 11 so sequence is 1
    assert_eq!(find_first_invalid_id_sequence(9), 1);

    // 12 has 2 digits (even) -> keep 12 -> first invalid 22 so sequence is 2
    assert_eq!(find_first_invalid_id_sequence(12), 2);

    // 999 has 3 digits (odd) -> round to 1000 -> first invalid 1010 so sequence is 10
    assert_eq!(find_first_invalid_id_sequence(999), 10);

    // 1234 has 4 digits (even) -> keep 1234 -> first invalid 1313 so sequence is 13
    assert_eq!(find_first_invalid_id_sequence(1234), 13);

    // 1 -> round to 10 -> first invalid 11 so sequence is 1
    assert_eq!(find_first_invalid_id_sequence(1), 1);

    // Large even-digit number -> first invalid 12351235 so sequence is 1235
    assert_eq!(find_first_invalid_id_sequence(12345678), 1235);
}

#[rstest]
#[case(11, 22, vec![11,22])]
#[case(95, 115, vec![99])]
#[case(998, 1012, vec![1010])]
#[case(1188511880, 1188511890, vec![1188511885])]
#[case(222220, 222224, vec![222222])]
#[case(1698522, 1698528, vec![])]
#[case(446443, 446449, vec![446446])]
#[case(38593856, 38593862, vec![38593859])]
#[case(69, 86, vec![77])]
fn test_examples_for_part_1(
    #[case] start_range: u128,
    #[case] end_range: u128,
    #[case] expected_ids: Vec<u128>
)
{
    // Act
    let invalid_ids = find_invalid_ids(start_range, end_range);

    // Assert
    assert_eq!(invalid_ids, expected_ids)
}

#[test]
fn check_part1_example() {
    // Arrange
    let path = "/workspaces/advent-of-code-2025-rust/day2-example.txt";

    // Act
    let password = find_password(path);

    // Assert
    assert_eq!(password, 1227775554);
}

#[test]
fn check_part1_input() {
    // Arrange
    let path = "/workspaces/advent-of-code-2025-rust/day2-input.txt";

    // Act
    let password = find_password(path);

    // Assert
    println!("Password: {}", password);
    assert!(password > 19717846043); // First guess too low
    assert_eq!(password, 30608905813);
}