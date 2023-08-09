use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "Spinlock";
const PROBLEM_INPUT_FILE: &str = "./input/day17.txt";
const PROBLEM_DAY: u64 = 17;

const PART1_CAP: usize = 2017;

/// Processes the AOC 2017 Day 17 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 17 input file in the format required by the solver functions.
///
/// Returned value is positive integer value given in the input file.
fn process_input_file(filename: &str) -> usize {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().parse::<usize>().unwrap()
}

/// Solves AOC 2017 Day 17 Part 1.
///
/// Identifies the value following 2017 in the spinlock circular buffer after 2017 insertions have
/// been completed.
fn solve_part1(steps: &usize) -> usize {
    let mut spinlock: Vec<usize> = vec![0];
    let mut cursor: usize = 0;
    for code in 1..=PART1_CAP {
        spinlock.insert(cursor + 1, code);
        cursor = (cursor + 1 + steps) % spinlock.len();
    }
    // Find the value after 2017 in the completed spinlock circular buffer
    let i_result = (spinlock.iter().position(|&v| v == PART1_CAP).unwrap() + 1) % spinlock.len();
    spinlock[i_result]
}

/// Solves AOC 2017 Day 17 Part 2.
///
/// ###
fn solve_part2(_cap: &usize) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 17 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day17_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(1642, solution);
    }

    /// Tests the Day 17 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day17_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(33601318, solution);
    }
}
