use crate::utils::read_file_lines;

#[allow(dead_code)]
pub fn main() {
    // let path = "/workspaces/advent-of-code-2025-rust/day6-example.txt";
    let path = "/workspaces/advent-of-code-2025-rust/day6-input.txt";

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

    let mut total = 0;
    for prob_idx in 0..num_problems {
        let operator = &matrix[num_arguments][prob_idx].chars().next().unwrap();
        let mut answer = match operator {            
                '*' => 1,
                '+' => 0,
                _ => panic!(),
        };
        for arg_idx in 0..num_arguments {
            let argument: u64 = (&matrix[arg_idx][prob_idx]).parse().unwrap();
            if (arg_idx > 0) {print!("{}",operator)}
            print!("{}",argument);
            answer = match operator {
                '*' => answer * argument,
                '+' => answer + argument,
                _ => panic!(),
            }

        }
        total += answer;
        println!("={}",answer);
    }
    println!("Grand Total: {}", total);
}
