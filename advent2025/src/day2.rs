use rstest::rstest;

pub fn main() {
    let range_start = 11;
    let range_end = 22;

    let invalid_ids = find_invalid_ids(range_start, range_end);

    println!("Invalid IDs: {:?}", invalid_ids)
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



/// Returns the larger half after:
/// 1) Rounding `n` up to the next integer with an even number of decimal digits.
/// 2) Splitting its decimal string into two equal halves.
///
/// Assumes `n >= 1`.
pub fn find_first_invalid_id_sequence(n: u128) -> u128 {
    let s = n.to_string();
    let digits = s.len();

    // If odd number of digits, round up to the smallest number with even digits:
    // This is 10^digits (e.g., 9 -> 10, 999 -> 1000).
    let m = if digits % 2 == 1 {
        // 10^digits fits in u128 as long as digits <= 38
        // (u128 max is 38 decimal digits). For larger inputs, this will panic.
        pow10_u128(digits as u32)
    } else {
        n
    };

    // Now split into two halves and compare numerically.
    let ms = m.to_string();
    let half = ms.len() / 2;
    let left = &ms[..half];
    let right = &ms[half..];

    let left_num = left.parse::<u128>().expect("left half must be numeric");
    let right_num = right.parse::<u128>().expect("right half must be numeric");

    left_num.max(right_num)
}

/// Integer power: 10^exp for u128.
fn pow10_u128(exp: u32) -> u128 {
    let mut acc: u128 = 1;
    for _ in 0..exp {
        acc *= 10;
    }
    acc
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
    // 9 has 1 digit (odd) -> round to 10 (even 2 digits) -> "10" -> halves: "1" and "0" -> max = 1
    assert_eq!(find_first_invalid_id_sequence(9), 1);

    // 12 has 2 digits (even) -> keep 12 -> "12" -> halves: "1" and "2" -> max = 2
    assert_eq!(find_first_invalid_id_sequence(12), 2);

    // 999 has 3 digits (odd) -> round to 1000 -> "1000" -> halves: "10" and "00" -> max = 10
    assert_eq!(find_first_invalid_id_sequence(999), 10);

    // 1234 has 4 digits (even) -> keep -> "1234" -> halves: "12" and "34" -> max = 34
    assert_eq!(find_first_invalid_id_sequence(1234), 34);

    // 1 -> round to 10 -> halves: "1" and "0" -> 1
    assert_eq!(find_first_invalid_id_sequence(1), 1);

    // Large even-digit number
    assert_eq!(find_first_invalid_id_sequence(12345678), 5678);
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