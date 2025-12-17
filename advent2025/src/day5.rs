use crate::utils::read_file_lines;
use rstest::rstest;

#[allow(dead_code)]
pub fn main() {
    //let path = "/workspaces/advent-of-code-2025-rust/day5-example.txt";
    let path = "/workspaces/advent-of-code-2025-rust/day5-input.txt";
    let fresh_items = find_fresh_item_count(path);
    println!("Number of fresh items: {}", fresh_items);
}

fn find_fresh_item_count(path: &str) -> u64 {
    let lines = read_file_lines(path).unwrap();
    let mut iter = lines.iter();

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