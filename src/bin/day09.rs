use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Stream Processing";
const PROBLEM_INPUT_FILE: &str = "./input/day09.txt";
const PROBLEM_DAY: u64 = 9;

/// Processes the AOC 2017 Day 09 input file and solves both parts of the problem. Solutions are
/// printed to stdout.
pub fn main() {
    let start = Instant::now();
    // Input processing
    let input = process_input_file(PROBLEM_INPUT_FILE);
    let input_parser_timestamp = Instant::now();
    let input_parser_duration = input_parser_timestamp.duration_since(start);
    // Solve part 1
    let p1_solution = solve_part1(&input);
    let p1_timestamp = Instant::now();
    let p1_duration = p1_timestamp.duration_since(input_parser_timestamp);
    // Solve part 2
    let p2_solution = solve_part2(&input);
    let p2_timestamp = Instant::now();
    let p2_duration = p2_timestamp.duration_since(p1_timestamp);
    // Print results
    println!("==================================================");
    println!("AOC 2017 Day {PROBLEM_DAY} - \"{PROBLEM_NAME}\"");
    println!("[+] Part 1: {p1_solution}");
    println!("[+] Part 2: {p2_solution}");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Execution times:");
    println!("[+] Input:  {input_parser_duration:.2?}");
    println!("[+] Part 1: {p1_duration:.2?}");
    println!("[+] Part 2: {p2_duration:.2?}");
    println!(
        "[*] TOTAL:  {:.2?}",
        input_parser_duration + p1_duration + p2_duration
    );
    println!("==================================================");
}

/// Processes the AOC 2017 Day 09 input file in the format required by the solver functions.
///
/// Returned value is vector of chars from input file.
fn process_input_file(filename: &str) -> Vec<char> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().chars().collect::<Vec<char>>()
}

/// Solves AOC 2017 Day 09 Part 1.
///
/// Calculates the total score for all groups in the character sequence.
fn solve_part1(chars: &[char]) -> u64 {
    let mut depth: u64 = 0;
    let mut score: u64 = 0;
    let mut cursor: usize = 0;
    let mut in_garbage = false;
    while cursor < chars.len() {
        match chars[cursor] {
            '{' => {
                if !in_garbage {
                    depth += 1;
                }
            }
            '}' => {
                if !in_garbage && depth > 0 {
                    score += depth;
                    depth -= 1;
                }
            }
            '<' => in_garbage = true,
            '>' => in_garbage = false,
            '!' => {
                if in_garbage {
                    cursor += 1;
                }
            }
            _ => (),
        }
        cursor += 1;
    }
    score
}

/// Solves AOC 2017 Day 09 Part 2.
///
/// Counts the number of non-cancelled characters within the garbage sections of the character
/// sequence.
fn solve_part2(chars: &[char]) -> u64 {
    let mut garbage_count: u64 = 0;
    let mut cursor: usize = 0;
    let mut in_garbage = false;
    while cursor < chars.len() {
        if in_garbage {
            garbage_count += 1;
        }
        match chars[cursor] {
            '<' => in_garbage = true,
            '>' => {
                in_garbage = false;
                garbage_count -= 1;
            }
            '!' => {
                cursor += 1;
                garbage_count -= 1;
            }
            _ => (),
        }
        cursor += 1;
    }
    garbage_count
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 09 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day09_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(16869, solution);
    }

    /// Tests the Day 09 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day09_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(7284, solution);
    }
}
