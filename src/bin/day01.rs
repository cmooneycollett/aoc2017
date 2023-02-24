use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "###";
const PROBLEM_INPUT_FILE: &str = "./input/day01.txt";
const PROBLEM_DAY: u64 = 1;

/// Processes the AOC 0000 Day 1 input file and solves both parts of the problem. Solutions are
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
    println!("AOC 0000 Day {} - \"{}\"", PROBLEM_DAY, PROBLEM_NAME);
    println!("[+] Part 1: {}", p1_solution);
    println!("[+] Part 2: {}", p2_solution);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Execution times:");
    println!("[+] Input:  {:.2?}", input_parser_duration);
    println!("[+] Part 1: {:.2?}", p1_duration);
    println!("[+] Part 2: {:.2?}", p2_duration);
    println!(
        "[*] TOTAL:  {:.2?}",
        input_parser_duration + p1_duration + p2_duration
    );
    println!("==================================================");
}

/// Processes the AOC 0000 Day 1 input file in the format required by the solver functions.
/// Returned value is ###.
fn process_input_file(filename: &str) -> String {
    // Read contents of problem input file
    let _raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    unimplemented!();
}

/// Solves AOC 0000 Day 1 Part 1 // ###
fn solve_part1(_input: &String) -> String {
    unimplemented!();
}

/// Solves AOC 0000 Day 1 Part 2 // ###
fn solve_part2(_input: &String) -> String {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 1 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day01_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part1(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }

    /// Tests the Day 1 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day01_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let _solution = solve_part2(&input);
        unimplemented!();
        // assert_eq!("###", solution);
    }
}
