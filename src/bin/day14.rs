use std::fs;
use std::num::ParseIntError;
use std::time::Instant;

use aoc2017::utils::knot_hash::calculate_knot_hash;

const PROBLEM_NAME: &str = "Disk Defragmentation";
const PROBLEM_INPUT_FILE: &str = "./input/day14.txt";
const PROBLEM_DAY: u64 = 14;

/// Processes the AOC 2017 Day 14 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 14 input file in the format required by the solver functions.
/// Returned value is string given in the input file.
fn process_input_file(filename: &str) -> String {
    // Read contents of problem input file
    fs::read_to_string(filename).unwrap().trim().to_string()
}

/// Solves AOC 2017 Day 14 Part 1 // Determines the number of squares used in the disk grid, with
/// rows based on knot hash calculations.
fn solve_part1(input: &str) -> usize {
    (0..=127)
        .map(|v| calculate_knot_hash(&format!("{input}-{v}")))
        .map(|s| convert_string_hexadecimal_to_binary(&s).unwrap())
        .map(|s| s.chars().filter(|c| *c == '1').count())
        .sum()
}

/// Solves AOC 2017 Day 14 Part 2 // ###
fn solve_part2(_input: &str) -> usize {
    unimplemented!();
}

/// Converts a hexadecimal string to its equivalent representation as a binary string (zero-padded).
fn convert_string_hexadecimal_to_binary(s: &str) -> Result<String, ParseIntError> {
    let mut binary_string = String::new();
    for c in s.chars() {
        let digit = u32::from_str_radix(&c.to_string(), 16)?;
        let binary_digit = format!("{digit:04b}");
        binary_string.push_str(&binary_digit);
    }
    Ok(binary_string)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 14 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day14_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(8190, solution);
    }

    /// Tests the Day 14 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day14_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(1134, solution);
    }
}
