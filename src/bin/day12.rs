use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Digital Plumber";
const PROBLEM_INPUT_FILE: &str = "./input/day12.txt";
const PROBLEM_DAY: u64 = 12;

lazy_static! {
    static ref INPUT_LINE_REGEX: Regex = Regex::new(r"^(\d+) <-> (.*)$").unwrap();
}

/// Custom error type indicating that the parsing of a line from the input file has failed.
#[derive(Debug)]
struct InputLineParseError;

/// Processes the AOC 2017 Day 12 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 12 input file in the format required by the solver functions.
/// Returned value is HashMap mapping each program to the other it is directly connected to via
/// pipes.
fn process_input_file(filename: &str) -> HashMap<u64, Vec<u64>> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut program_conns: HashMap<u64, Vec<u64>> = HashMap::new();
    for line in raw_input.lines() {
        if let Ok((left, right)) = parse_input_file_line(line) {
            program_conns.insert(left, right);
        }
    }
    program_conns
}

/// Solves AOC 2017 Day 12 Part 1 // Determines the number of programs in the group containing the
/// program '0'.
fn solve_part1(input: &HashMap<u64, Vec<u64>>) -> usize {
    determine_program_group_members(0, input).len()
}

/// Solves AOC 2017 Day 12 Part 2 // Determines the total number of separate programs groups
/// specified in the program connections.
fn solve_part2(input: &HashMap<u64, Vec<u64>>) -> usize {
    let mut visited: HashSet<u64> = HashSet::new();
    let mut total_program_groups = 0;
    for program in input.keys() {
        // Skip any programs that have observed in a previously visited group
        if !visited.contains(program) {
            // Update the overall record of programs visited to prevent double-counting
            let run_visited = determine_program_group_members(*program, input);
            visited.extend(run_visited);
            total_program_groups += 1;
        }
    }
    total_program_groups
}

/// Parses one line from the input file to extract the left program and its connected right
/// programs.
fn parse_input_file_line(s: &str) -> Result<(u64, Vec<u64>), InputLineParseError> {
    if let Ok(Some(caps)) = INPUT_LINE_REGEX.captures(s) {
        let left = caps[1].parse::<u64>().unwrap();
        let right = caps[2]
            .split(',')
            .map(|v| v.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        return Ok((left, right));
    }
    Err(InputLineParseError)
}

/// Determines the members of the program group containing the start program.
fn determine_program_group_members(
    start: u64,
    program_conns: &HashMap<u64, Vec<u64>>,
) -> HashSet<u64> {
    let mut visited: HashSet<u64> = HashSet::new();
    let mut visit_queue: VecDeque<u64> = VecDeque::from([start]);
    while !visit_queue.is_empty() {
        let program = visit_queue.pop_front().unwrap();
        visited.insert(program);
        if let Some(conns) = program_conns.get(&program) {
            for next in conns {
                if !visited.contains(next) {
                    visit_queue.push_back(*next);
                }
            }
        }
    }
    visited
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 12 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day12_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(288, solution);
    }

    /// Tests the Day 12 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day12_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(211, solution);
    }
}
