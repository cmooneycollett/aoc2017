use std::fs;
use std::time::Instant;

use itertools::{iproduct, Itertools};

const PROBLEM_NAME: &str = "High-Entropy Passphrases";
const PROBLEM_INPUT_FILE: &str = "./input/day04.txt";
const PROBLEM_DAY: u64 = 4;

/// Processes the AOC 2017 Day 04 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 04 input file in the format required by the solver functions.
/// Returned value is vector of containing vector of words separated by whitespace in the input file
/// lines.
fn process_input_file(filename: &str) -> Vec<Vec<String>> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .map_into::<String>()
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>()
}

/// Solves AOC 2017 Day 04 Part 1 // Counts the number of passphrases that do not no contain any
/// duplicate words.
fn solve_part1(passphrases: &[Vec<String>]) -> usize {
    passphrases
        .iter()
        .filter(|pass| pass.len() == pass.iter().unique().count())
        .count()
}

/// Solves AOC 2017 Day 04 Part 2 // Counts the number of passphrases that do not contain two
/// strings which are anagrams of each other.
fn solve_part2(passphrases: &[Vec<String>]) -> usize {
    passphrases
        .iter()
        .filter(|pass| {
            iproduct!(pass.iter().enumerate(), pass.iter().enumerate())
                .filter(|((i, left), (j, right))| i != j && check_anagram(left, right))
                .count()
                == 0
        })
        .count()
}

/// Checks if the left and right strings are anagrams of each other.
fn check_anagram(left: &str, right: &str) -> bool {
    let left_chars = left.chars().sorted().collect::<Vec<char>>();
    let right_chars = right.chars().sorted().collect::<Vec<char>>();
    left_chars == right_chars
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 04 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day04_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(386, solution);
    }

    /// Tests the Day 04 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day04_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(208, solution);
    }
}
