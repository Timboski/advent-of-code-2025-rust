

pub fn main() {
    let input: Vec<u8> = "987654321111111".chars()
        .map(|ch| ch
        .to_digit(10)
        .map(|d| d as u8)
        .unwrap())
        .collect();

    let result = max_and_tail_after_first_max_ignore_last(&input, 1).unwrap();
    let first_number = result.0;
    let second_number = result.1.iter().max().unwrap();
    let joltage = first_number * 10 + second_number;
    println!("Joltage of {:?} is {} and {} equals {}", input, first_number, second_number, joltage);

}

/// Finds the maximum within the first `len - x` elements and returns:
/// - the max value (`i8`)
/// - a Vec<i8> containing all elements **after the first occurrence** of that max,
///   taken from the **entire original vector** (including the ignored last `x` elements).
///
/// Returns:
/// - None if there’s no usable prefix (i.e., nums.is_empty() or x >= nums.len()).
///
/// Notes:
/// - The max and its first index are determined **only** over the prefix `&nums[..len - x]`.
/// - The tail is taken from `nums[(max_idx + 1)..]` (full vector, not truncated).
fn max_and_tail_after_first_max_ignore_last(nums: &[u8], x: usize) -> Option<(u8, Vec<u8>)> {
    let len = nums.len();
    if x >= len {
        return None; // no prefix to compute a max over
    }

    let prefix_len = len - x;
    let prefix = &nums[..prefix_len];
    debug_assert!(!prefix.is_empty());

    // Find max and its first index in the prefix
    let mut max_val = prefix[0];
    let mut max_idx = 0usize;
    for (i, &v) in prefix.iter().enumerate() {
        if v > max_val {
            max_val = v;
            max_idx = i;
        }
    }

    // Tail includes everything after that index in the **entire** vector
    let tail = if max_idx + 1 < len {
        nums[(max_idx + 1)..].to_vec()
    } else {
        Vec::new()
    };

    Some((max_val, tail))
}


#[test]
fn test_max_and_tail_after_first_max_ignore_last() {
    // Assertions covering behavior
    assert_eq!(max_and_tail_after_first_max_ignore_last(&[], 0), None);
    assert_eq!(
        max_and_tail_after_first_max_ignore_last(&[1, 5, 2, 5, 3], 0),
        Some((5, vec![2, 5, 3])) // after first 5: includes rest of vector
    );
    assert_eq!(
        max_and_tail_after_first_max_ignore_last(&[1, 5, 2, 5, 3], 2),
        // we compute max over [1,5,2], first max at index 1 -> tail from index 2 to end of full vec
        Some((5, vec![2, 5, 3]))
    );
    assert_eq!(
        max_and_tail_after_first_max_ignore_last(&[9, 1, 2], 1),
        // compute over [9,1], first max at 0 -> tail is [1,2] (includes ignored suffix)
        Some((9, vec![1, 2]))
    );
    assert_eq!(
        max_and_tail_after_first_max_ignore_last(&[1, 2, 9], 0),
        // first max at index 2 -> tail empty
        Some((9, vec![]))
    );
    assert_eq!(
        max_and_tail_after_first_max_ignore_last(&[1, 2, 9], 1),
        // compute over [1,2], max at index 1 -> tail from index 2 to end: [9]
        Some((2, vec![9]))
    );
    assert_eq!(
        max_and_tail_after_first_max_ignore_last(&[1, 2, 9], 3),
        None
    );
}
