use rstest::rstest;
use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn main() {
    let path = "/workspaces/advent-of-code-2025-rust/day2-input.txt";

    let sum = find_password(path);

    println!("Password: {}", sum);
}

fn find_password(path: &str) -> u128 {
    let ranges = read_ranges_from_file(path);

    let mut sum: u128 = 0;
    for range in ranges {
        let invalid_ids = find_invalid_ids(range.start, range.end, 2);
        println!("Start {}, End {}, Invalid IDs: {:?}", range.start, range.end, invalid_ids);
        let total: u128 = invalid_ids.iter().sum();
        sum += total;
    }

    sum
}

fn find_invalid_ids(range_start: u128, range_end: u128, repeat_count: usize) -> Vec<u128> {
    let start = find_first_invalid_id_sequence(range_start, repeat_count);
    let mut invalid_ids: Vec<u128> = Vec::new();
    
    let mut current = start;
    loop {
        let invalid_id = duplicate_block(current, repeat_count);
        if invalid_id > range_end { break; }
        invalid_ids.push(invalid_id);
        current += 1;
    }
    
    invalid_ids
}

pub struct Range {
    pub start: u128,
    pub end: u128,
}

pub fn read_ranges_from_file(path: &str) -> Vec<Range> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let _ = reader.read_line(&mut line);

    let mut out = Vec::new();

    for range in line.split(',') {
        let (start, end) = range.split_once('-').unwrap();
        out.push(Range {
            start: start.parse::<u128>().unwrap(),
            end: end.parse::<u128>().unwrap(),
        });
    }

    out
}

pub fn find_first_invalid_id_sequence(range_start: u128, repeat_count: usize) -> u128 {
    let s = range_start.to_string();
    let digits = s.len();
    
    let first_sequence: u128 = if digits % repeat_count != 0 {
        // Odd number of digits. Round up to the smallest number with even digits (and chop in half)
        // This is the same as 1 followed by a zero for each of half the original number of digits (rounded down).
        let zeros_count = digits / repeat_count;
        let even_start = format!("1{}", "0".repeat(zeros_count));
        even_start.parse::<u128>().unwrap()
    } else {
        // Just grab the first digits using string manipulation.
        let range_start_as_string = range_start.to_string();
        let size = range_start_as_string.len() / repeat_count;
        let sequence = &range_start_as_string[..size];
        sequence.parse::<u128>().unwrap()
    };

    // If the range_start is not an invalid id, the duplicated sequence will be below the range.
    // Incrementing by one will bring it into (or over) the range.
    if duplicate_block(first_sequence, repeat_count) < range_start { return first_sequence + 1; }

    // The range start is an invalid id, so ensure this is included.
    return first_sequence;
}

fn duplicate_block(sequence: u128, repeat_count: usize) -> u128 {
    let s = sequence.to_string();
    s.repeat(repeat_count).parse::<u128>().ok().unwrap()
}


#[test]
pub fn test_find_first_invalid_id_sequence() {
    // 9 has 1 digit (odd) -> round to 10 (even 2 digits) -> first invalid 11 so sequence is 1
    assert_eq!(find_first_invalid_id_sequence(9, 2), 1);

    // 12 has 2 digits (even) -> keep 12 -> first invalid 22 so sequence is 2
    assert_eq!(find_first_invalid_id_sequence(12, 2), 2);

    // 999 has 3 digits (odd) -> round to 1000 -> first invalid 1010 so sequence is 10
    assert_eq!(find_first_invalid_id_sequence(999, 2), 10);

    // 1234 has 4 digits (even) -> keep 1234 -> first invalid 1313 so sequence is 13
    assert_eq!(find_first_invalid_id_sequence(1234, 2), 13);

    // 1 -> round to 10 -> first invalid 11 so sequence is 1
    assert_eq!(find_first_invalid_id_sequence(1, 2), 1);

    // Large even-digit number -> first invalid 12351235 so sequence is 1235
    assert_eq!(find_first_invalid_id_sequence(12345678, 2), 1235);

    assert_eq!(find_first_invalid_id_sequence(95, 3), 1); // 111
    assert_eq!(find_first_invalid_id_sequence(998, 3), 9); // 999
    assert_eq!(find_first_invalid_id_sequence(222220, 6), 2); // 222222
    assert_eq!(find_first_invalid_id_sequence(222220, 3), 22); // 222222
    assert_eq!(find_first_invalid_id_sequence(222220, 2), 222); // 222222
    assert_eq!(find_first_invalid_id_sequence(565653, 3), 56); // 565656
    assert_eq!(find_first_invalid_id_sequence(824824821, 3), 824); // 824824824
    assert_eq!(find_first_invalid_id_sequence(2121212118, 5), 21); // 2121212121
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
    let invalid_ids = find_invalid_ids(start_range, end_range, 2);

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