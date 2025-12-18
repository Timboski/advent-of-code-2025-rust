use crate::utils::read_file_lines;
use rstest::rstest;

#[allow(dead_code)]
pub fn main() {
    let path = "/workspaces/advent-of-code-2025-rust/day6-example.txt";
    // let path = "/workspaces/advent-of-code-2025-rust/day6-input.txt";

    let total1 = part1(path);
    //let total2 = part2(path);

    println!();
    println!("Grand Total (Part1): {}", total1);
    //println!("Grand Total (Part2): {}", total2);
}

fn part1(path: &str) -> u64 {
    let problems = decode_puzzle_input(path);

    let mut total = 0;
    for problem in problems {
        total += solver(problem)
    }

    total
}

fn decode_puzzle_input(path: &str) -> Vec<(char, Vec<String>)> {
    let lines = read_file_lines(path).unwrap();
    let num_arguments = lines.len() - 1;
    println!("Size of problems: {}", num_arguments);

    let mut matrix: Vec<Vec<String>> = Vec::new();

    for line in lines {
        let elements: Vec<String> = line
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        matrix.push(elements);
    }

    let num_problems = matrix[0].len();
    println!("Number of problems: {}", num_problems);

    let mut problems: Vec<(char, Vec<String>)> = Vec::new();
    for prob_idx in 0..num_problems {
        let operator = matrix[num_arguments][prob_idx].chars().next().unwrap();
        let mut arguments = Vec::new();
        for arg_idx in 0..num_arguments {
            arguments.push(matrix[arg_idx][prob_idx].clone());
        }
        problems.push((operator, arguments));
    }

    problems
}

fn solver(problem: (char, Vec<String>)) -> u64 {
    let mut answer = match problem.0 {            
            '*' => 1,
            '+' => 0,
            _ => panic!(),
    };
    for arg_idx in 0..problem.1.len() {
        let argument: u64 = (problem.1[arg_idx]).parse().unwrap();
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