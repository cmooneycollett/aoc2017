use std::fs;
use std::time::Instant;

use itertools::iproduct;

const PROBLEM_NAME: &str = "Corruption Checksum";
const PROBLEM_INPUT_FILE: &str = "./input/day02.txt";
const PROBLEM_DAY: u64 = 2;

/// Processes the AOC 2017 Day 02 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 02 input file in the format required by the solver functions.
/// Returned value is vector of vectors containing values given in the lines of the input file.
fn process_input_file(filename: &str) -> Vec<Vec<u64>> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|elem| elem.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>()
}

/// Solves AOC 2017 Day 02 Part 1 // Determines the checksum of the sheet by determining the sum of
/// the difference between the minimum and maximum values for each row.
fn solve_part1(sheet: &[Vec<u64>]) -> u64 {
    sheet
        .iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}

/// Solves AOC 2017 Day 02 Part 2 // Determines the checksum of the sheet by finding the sum of the
/// division result of the two values from each row that are evenly divisible.
fn solve_part2(sheet: &[Vec<u64>]) -> u64 {
    sheet
        .iter()
        .map(|row| {
            iproduct!(row.iter(), row.iter())
                .filter(|(&a, &b)| a != b && a % b == 0)
                .map(|(&a, &b)| a / b)
                .next()
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 02 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day02_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(45158, solution);
    }

    /// Tests the Day 02 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day02_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(294, solution);
    }
}
