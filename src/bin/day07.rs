use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;

const PROBLEM_NAME: &str = "Recursive Circus";
const PROBLEM_INPUT_FILE: &str = "./input/day07.txt";
const PROBLEM_DAY: u64 = 7;

/// Custom type for problem input generated from parsing input file. First element is hashmap
/// mapping program name to weight as given in input file. Second element is hashmap mapping program
/// to collection of other program names sitting on top of the program.
type ProblemInput = (HashMap<String, u64>, HashMap<String, Vec<String>>);

/// Custom error type indicating a failure to process the program tree information generated from
/// the input file.
#[derive(Debug)]
struct ProgramTreeProcessingError;

/// Processes the AOC 2017 Day 07 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 07 input file in the format required by the solver functions.
/// Returned value is tuple containing: hashmap mapping program name to weight as given in input
/// file, and hashmap mapping program to collection of other program names sitting on top of the
/// program.
fn process_input_file(filename: &str) -> ProblemInput {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let regex_line = Regex::new(r"^([a-z]+) \((\d+)\)(?: -> )?(.+)?$").unwrap();
    let mut program_weights: HashMap<String, u64> = HashMap::new();
    let mut program_children: HashMap<String, Vec<String>> = HashMap::new();
    for line in raw_input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
    {
        if let Ok(Some(caps)) = regex_line.captures(line) {
            // Filter out non-matched optional capture groups
            let caps = caps
                .iter()
                .filter(|cap| cap.is_some())
                .map(|cap| cap.unwrap().as_str())
                .collect::<Vec<&str>>();
            let program = &caps[1];
            let weight = caps[2].parse::<u64>().unwrap();
            let children: Vec<String> = {
                if caps.len() == 4 {
                    caps[3]
                        .trim()
                        .split(", ")
                        .map(|name| name.to_string())
                        .collect::<Vec<String>>()
                } else {
                    vec![]
                }
            };
            program_weights.insert(program.to_string(), weight);
            program_children.insert(program.to_string(), children);
        } else {
            panic!("Bad format input line! // {line}");
        }
    }
    (program_weights, program_children)
}

/// Solves AOC 2017 Day 07 Part 1 // Determines the name of the program at the bottom of the tower.
fn solve_part1(input: &ProblemInput) -> String {
    let (_, program_children) = input;
    match find_bottom_program_name(program_children) {
        Ok(name) => name,
        Err(ProgramTreeProcessingError) => panic!("Failed to find the name of the bottom program"),
    }
}

/// Solves AOC 2017 Day 07 Part 2 // ###
fn solve_part2(_input: &ProblemInput) -> u64 {
    0
}

/// Finds the name of the bottom program (the only program that is not on top of another program).
///
/// Returns [`ProgramTreeProcessingError`] if there is no bottom program found.
fn find_bottom_program_name(
    program_children: &HashMap<String, Vec<String>>,
) -> Result<String, ProgramTreeProcessingError> {
    let children = program_children
        .values()
        .flat_map(|vec| vec.iter())
        .collect::<HashSet<&String>>();
    for name in program_children.keys() {
        if !children.contains(name) {
            return Ok(name.to_string());
        }
    }
    // Failed to find the name of the bottom program
    Err(ProgramTreeProcessingError)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 07 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day07_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!("hlqnsbe", solution);
    }

    /// Tests the Day 07 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day07_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(1993, solution);
    }
}
