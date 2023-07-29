use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Packet Scanners";
const PROBLEM_INPUT_FILE: &str = "./input/day13.txt";
const PROBLEM_DAY: u64 = 13;

lazy_static! {
    static ref INPUT_LINE_REGEX: Regex = Regex::new(r"^(\d+): (\d+)$").unwrap();
}

/// Custom error type indicating that the parsing of a line from the input file has failed.
#[derive(Debug)]
struct InputLineParseError;

/// Processes the AOC 2017 Day 13 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 13 input file in the format required by the solver functions.
/// Returned value is HashMap mapping the depth of each firewall to its range.
fn process_input_file(filename: &str) -> HashMap<u64, u64> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .lines()
        .filter_map(|s| parse_input_file_line(s).ok())
        .collect::<HashMap<u64, u64>>()
}

/// Solves AOC 2017 Day 13 Part 1 // Determines the severity score for the trip if there is no delay
/// before commencement of the firewall transit.
fn solve_part1(input: &HashMap<u64, u64>) -> u64 {
    input
        .iter()
        .filter(|(depth, range)| *depth % (2 * (*range - 1)) == 0)
        .map(|(depth, range)| depth * range)
        .sum()
}

/// Solves AOC 2017 Day 13 Part 2 // Determines the total delay (in picoseconds) prior to
/// commencement required to complete the firewall transit without being caught.
fn solve_part2(input: &HashMap<u64, u64>) -> u64 {
    let mut delay_ps = 0;
    loop {
        // For each firewall, check if we will collide with its detector (at the top of range)
        let mut caught = false;
        for (depth, range) in input.iter() {
            let cycle = 2 * (range - 1);
            if (depth + delay_ps) % cycle == 0 {
                caught = true;
                break;
            }
        }
        // We have successfully completed the transit without being caught
        if !caught {
            break;
        }
        // We were caught, so we need to increase our delay and reattempt the transit
        delay_ps += 1;
    }
    delay_ps
}

/// Parses a single line from the input file to extract required values.
///
/// If Ok() is returned, the wrapped value represents the depth and range of the firewall specified
/// by the file line. Otherwise, an [`InputLineParseError`] is returned.
fn parse_input_file_line(s: &str) -> Result<(u64, u64), InputLineParseError> {
    if let Ok(Some(caps)) = INPUT_LINE_REGEX.captures(s) {
        let depth = caps[1].parse::<u64>().unwrap();
        let range = caps[2].parse::<u64>().unwrap();
        return Ok((depth, range));
    }
    Err(InputLineParseError)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 13 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day13_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(2160, solution);
    }

    /// Tests the Day 13 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day13_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(3907470, solution);
    }
}
