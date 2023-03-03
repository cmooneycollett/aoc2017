use std::collections::hash_map::Entry;
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

/// Solves AOC 2017 Day 07 Part 2 // Given that exactly one program is the wrong weight, finds the
/// weight that it would need to be to balance the entire program tower.
fn solve_part2(input: &ProblemInput) -> u64 {
    let (program_weights, parent_to_children) = input;
    match find_unbalanced_program_corrected_weight(program_weights, parent_to_children) {
        Ok(correct_weight) => correct_weight,
        Err(ProgramTreeProcessingError) => {
            panic!("Failed to find corrected weight! Program tower is already balanced.")
        }
    }
}

/// Finds the name of the bottom program (the first program that is not on top of another program).
///
/// Returns [`ProgramTreeProcessingError`] if there is no bottom program found.
fn find_bottom_program_name(
    program_children: &HashMap<String, Vec<String>>,
) -> Result<String, ProgramTreeProcessingError> {
    let children = program_children
        .values()
        .flat_map(|vec| vec.iter())
        .collect::<HashSet<&String>>();
    let names = program_children.keys().collect::<HashSet<&String>>();
    if let Some(bottom_name) = names.difference(&children).next() {
        return Ok(bottom_name.to_string());
    }
    // Failed to find the name of the bottom program
    Err(ProgramTreeProcessingError)
}

/// Converts the mapping of parent-to-children programs (one to many) into a mapping of
/// child-to-parent (one-to-one) programs.
fn generate_child_to_parent_mapping(
    program_children: &HashMap<String, Vec<String>>,
) -> HashMap<String, String> {
    let mut program_parents: HashMap<String, String> = HashMap::new();
    for (parent, children) in program_children {
        for child in children {
            program_parents.insert(child.to_string(), parent.to_string());
        }
    }
    program_parents
}

/// Finds the corrected weight for the one program in the tower that is the incorrect weight.
///
/// Returns [`ProgramTreeProcessingError`] if the program tree is already balanced.
fn find_unbalanced_program_corrected_weight(
    program_weights: &HashMap<String, u64>,
    parent_to_children: &HashMap<String, Vec<String>>,
) -> Result<u64, ProgramTreeProcessingError> {
    let child_to_parent = generate_child_to_parent_mapping(parent_to_children);
    let mut calculated_weights: HashMap<String, u64> = HashMap::new();
    for name in parent_to_children
        .iter()
        .filter(|(_p, c)| c.is_empty())
        .map(|(p, _c)| p)
    {
        if let Some(corrected_weight) = find_unbalanced_program_corrected_weight_recursive(
            name,
            program_weights,
            parent_to_children,
            &child_to_parent,
            &mut calculated_weights,
        ) {
            return Ok(corrected_weight);
        }
    }
    Err(ProgramTreeProcessingError)
}

/// Recursive helper function for finding the corrected weight of the one program in the tower with
/// the incorrect weight.
///
/// Returns [`ProgramTreeProcessingError`] if the bottom program is reached before the program with
/// the incorrect weight is found. This is the case when the program tower is already balanced.
fn find_unbalanced_program_corrected_weight_recursive(
    current_program: &str,
    program_weights: &HashMap<String, u64>,
    parent_to_children: &HashMap<String, Vec<String>>,
    child_to_parent: &HashMap<String, String>,
    calculated_weights: &mut HashMap<String, u64>,
) -> Option<u64> {
    // Check if we are on a top program
    if parent_to_children.get(current_program).unwrap().is_empty() {
        calculated_weights.insert(
            current_program.to_string(),
            *program_weights.get(current_program).unwrap(),
        );
    } else {
        // check if all child programs have been visited
        for child in parent_to_children.get(current_program).unwrap() {
            if !calculated_weights.contains_key(child) {
                return None;
            }
        }
        let mut current_program_weight = 0;
        let mut weight_occurrences: HashMap<u64, u64> = HashMap::new();
        let mut child_weight_record: HashMap<String, u64> = HashMap::new();
        for child in parent_to_children.get(current_program).unwrap() {
            let child_weight = *calculated_weights.get(child).unwrap();
            child_weight_record.insert(child.to_string(), child_weight);
            current_program_weight += child_weight;
            if let Entry::Vacant(e) = weight_occurrences.entry(child_weight) {
                e.insert(1);
            } else {
                *weight_occurrences.get_mut(&child_weight).unwrap() += 1;
            }
        }
        // Check if mismatched weight found
        let valid_weight = *weight_occurrences
            .iter()
            .max_by_key(|entry| entry.1)
            .unwrap()
            .0;
        for (child_name, weight) in child_weight_record {
            if weight != valid_weight {
                let delta = u64::abs_diff(valid_weight, weight);
                let child_weight = program_weights.get(&child_name).unwrap();
                let corrected_weight = if valid_weight > weight {
                    child_weight + delta
                } else {
                    child_weight - delta
                };
                return Some(corrected_weight);
            }
        }
        // Record calculated weight of current program
        current_program_weight += program_weights.get(current_program).unwrap();
        calculated_weights.insert(current_program.to_string(), current_program_weight);
    }
    // Go to the parent program
    if let Some(parent) = child_to_parent.get(current_program) {
        find_unbalanced_program_corrected_weight_recursive(
            parent,
            program_weights,
            parent_to_children,
            child_to_parent,
            calculated_weights,
        )
    } else {
        None
    }
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
