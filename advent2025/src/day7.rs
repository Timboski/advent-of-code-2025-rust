use crate::utils::read_file_lines;
use rstest::rstest;

#[allow(dead_code)]
pub fn main() {
    let path = "/workspaces/advent-of-code-2025-rust/day7-example.txt";
    // let path = "/workspaces/advent-of-code-2025-rust/day7-input.txt";

    let (splits, paths) = trace_beam_path(path);

    println!("Number of splits: {}", splits);
    println!("Number of paths: {}", paths);
}

fn trace_beam_path(path: &str) -> (u32, u64) {
    let input = read_file_lines(path).unwrap();

    let mut iter = input.iter();
    let mut prev = iter.next().unwrap().chars().map(|c| match c {'.' => 0, 'S' => 1, _ => panic!()}).collect();

    println!("{:?}", prev);
    let mut splits = 0;
    for current in iter {
        let path = compute_path(&prev, current);
        println!("{:?}", path.0);

        splits += path.1;
        prev = path.0;
    }

    (splits, prev.iter().sum::<u64>())
}

fn compute_path(prev: &Vec<u64>, current: &String) -> (Vec<u64>, u32) {
    let current_chars: Vec<char> = current.chars().collect();
    let mut new_line = vec![0; current.len()];

    let mut num_splits = 0;
    for i in 0..prev.len() {
        match prev[i] {
            0 => {}, 
            n => match current_chars[i] {
                '.' => new_line[i] += n,
                '^' => {
                    num_splits += 1;
                    new_line[i-1] += n;
                    new_line[i+1] += n;
                },
                _ => panic!(),
            },
            _ => {}
        }
    };

    (new_line.into_iter().collect(), num_splits)
}


#[rstest]
#[case("/workspaces/advent-of-code-2025-rust/day7-example.txt", 21, 40)]
#[case("/workspaces/advent-of-code-2025-rust/day7-input.txt", 1560, 25592971184998)]
fn test_answers(
    #[case] path: &str,
    #[case] expected_splits: u32,
    #[case] expected_paths: u64
)
{
    // Act
    let (splits, paths) = trace_beam_path(path);

    // Assert
    assert_eq!(splits, expected_splits);
    assert_eq!(paths, expected_paths);
}