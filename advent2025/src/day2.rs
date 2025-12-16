
// Prompt:
// using rust take a positive integer and round up to the next integer with an even number of decimal digits, Then take the number as a string and split into two halves. Return the larger of the two numbers.


pub fn main() {
    let start = find_first_invalid_id_sequence(11);
    let end = find_first_invalid_id_sequence(23);

    let mut current = start;
    while current < end {
        let invalid_id = duplicate_block(current);
        println!("{}", invalid_id);
        current += 1;
    }
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