use crate::utils::read_file_lines;
use rstest::rstest;

#[allow(dead_code)]
pub fn main() {
    //let path = "/workspaces/advent-of-code-2025-rust/day5-example.txt";
    let path = "/workspaces/advent-of-code-2025-rust/day5-input.txt";

    println!("Part 1");
    let fresh_items_in_list = find_fresh_item_count(path);

    println!();
    println!("Part 2");
    let total_fresh_items = find_total_number_of_fresh_items(path);
    
    println!();
    println!("Number of fresh items in list: {}", fresh_items_in_list);
    println!("Total number of fresh items: {}", total_fresh_items);
}

fn find_fresh_item_count(path: &str) -> u64 {
    let lines = read_file_lines(path).unwrap();
    let mut iter = lines.iter();

    let in_date_ranges = read_date_ranges(&mut iter);
    
        // Read the items
    let mut count = 0;
    for item in iter.map(|i| i.parse::<u64>().unwrap()) {
        print!("Item {:?}", item);
        let mut fresh = false;
        for range in &in_date_ranges {
            if item >= range.start && item <= range.end {
                fresh = true;
                continue;
            }
        }
        if fresh { 
            println!(" is FRESH");
            count += 1;
        }
        else {println!(" is SPOILED")}
    }

    count
}

fn read_date_ranges(iter: &mut std::slice::Iter<'_, String>) -> Vec<Range> {
    // Read the ranges
    let mut in_date_ranges = Vec::new();
    for line in iter.by_ref().take_while(|s| !s.is_empty()) {
        print!("{}", line);
        let (start, end) = line.split_once('-').unwrap();
        in_date_ranges.push(Range {
            start: start.parse::<u64>().unwrap(),
            end: end.parse::<u64>().unwrap(),
        });
        println!(" => {}-{}", start, end);
    }
    println!();
    in_date_ranges
}

fn find_total_number_of_fresh_items(path: &str) -> u64 {
    let lines = read_file_lines(path).unwrap();

    // Read the ranges
    let mut in_date_ranges = read_date_ranges(&mut lines.iter());

    // Sort the ranges
    in_date_ranges.sort_by_key(|r| r.start);

    let mut count = 0;
    let mut start = in_date_ranges.first().unwrap().start;
    let mut end = in_date_ranges.first().unwrap().end;
    for range in in_date_ranges {
        if range.start <= end {
            // We can combine these ranges
            print!("Combining range {}-{} with {}-{}", start, end, range.start, range.end);
            if range.end > end {end = range.end}
            println!(" => {}-{}", start, end);
        }
        else {
            // We are starting a new range
            let range_size = end - start + 1;
            println!("Range {}-{} -> items {}", start, end, range_size);
            start = range.start;
            end = range.end;
            count += range_size;
        }
    }

    // Add the last range
    let range_size = end - start + 1;
    println!("Range {}-{} -> items {}", start, end, range_size);
    count += range_size;

    // Read the items
    count
}
struct Range {
    start: u64,
    end: u64,
}

#[rstest]
#[case("/workspaces/advent-of-code-2025-rust/day5-example.txt", 3)]
#[case("/workspaces/advent-of-code-2025-rust/day5-input.txt", 874)]
fn test_part1_answers(
    #[case] path: &str,
    #[case] expected_fresh_items: u64
)
{
    // Act
    let fresh_items = find_fresh_item_count(path);

    // Assert
    assert_eq!(fresh_items, expected_fresh_items);
}

#[rstest]
#[case("/workspaces/advent-of-code-2025-rust/day5-example.txt", 14)]
#[case("/workspaces/advent-of-code-2025-rust/day5-input.txt", 348548952146313)]
fn test_part2_answers(
    #[case] path: &str,
    #[case] expected_fresh_items: u64
)
{
    // Act
    let fresh_items = find_total_number_of_fresh_items(path);

    // Assert
    assert_eq!(fresh_items, expected_fresh_items);
}

#[test]
fn test_part2()
{
    // Arrange
    let path = "/workspaces/advent-of-code-2025-rust/day5-input.txt";

    // Act
    let fresh_items = find_total_number_of_fresh_items(path);

    // Assert
    assert!(fresh_items > 325794691374101); // First guess is too low
}