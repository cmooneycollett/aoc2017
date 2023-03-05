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

/// Custom error type indicating a failure to process the program tower information generated from
/// the input file.
#[derive(Debug)]
struct ProgramTowerProcessingError;

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
        Err(ProgramTowerProcessingError) => panic!("Failed to find the name of the bottom program"),
    }
}

/// Solves AOC 2017 Day 07 Part 2 // Given that exactly one program is the wrong weight, finds the
/// weight that it would need to be to balance the entire program tower.
fn solve_part2(input: &ProblemInput) -> u64 {
    let (program_weights, parent_to_children) = input;
    match find_unbalanced_program_corrected_weight(program_weights, parent_to_children) {
        Ok(correct_weight) => correct_weight,
        Err(ProgramTowerProcessingError) => {
            panic!("Failed to find corrected weight! Program tower is already balanced.")
        }
    }
}

/// Finds the name of the bottom program (the first program that is not on top of another program).
///
/// Returns [`ProgramTowerProcessingError`] if there is no bottom program found.
fn find_bottom_program_name(
    program_children: &HashMap<String, Vec<String>>,
) -> Result<String, ProgramTowerProcessingError> {
    let children = program_children
        .values()
        .flat_map(|vec| vec.iter())
        .collect::<HashSet<&String>>();
    let names = program_children.keys().collect::<HashSet<&String>>();
    if let Some(bottom_name) = names.difference(&children).next() {
        return Ok(bottom_name.to_string());
    }
    // Failed to find the name of the bottom program
    Err(ProgramTowerProcessingError)
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
/// Returns [`ProgramTowerProcessingError`] if the program tower is already balanced.
fn find_unbalanced_program_corrected_weight(
    program_weights: &HashMap<String, u64>,
    parent_to_children: &HashMap<String, Vec<String>>,
) -> Result<u64, ProgramTowerProcessingError> {
    let child_to_parent = generate_child_to_parent_mapping(parent_to_children);
    let mut tower_weights: HashMap<String, u64> = HashMap::new();
    // Enter the program tower from each of the leaf programs (those with no parents)
    for current_program in parent_to_children
        .iter()
        .filter(|(_p, c)| c.is_empty())
        .map(|(p, _c)| p)
    {
        // Check if ok value returned
        if let Some(corrected_weight) = find_unbalanced_program_corrected_weight_recursive(
            current_program,
            program_weights,
            parent_to_children,
            &child_to_parent,
            &mut tower_weights,
        ) {
            return Ok(corrected_weight);
        }
    }
    Err(ProgramTowerProcessingError)
}

/// Recursive helper function for finding the corrected weight of the one program in the tower with
/// the incorrect weight.
///
/// Returns [`ProgramTowerProcessingError`] if the bottom program is reached before the program with
/// the incorrect weight is found. This is the case when the program tower is already balanced.
fn find_unbalanced_program_corrected_weight_recursive(
    current_program: &str,
    program_weights: &HashMap<String, u64>,
    parent_to_children: &HashMap<String, Vec<String>>,
    child_to_parent: &HashMap<String, String>,
    tower_weights: &mut HashMap<String, u64>,
) -> Option<u64> {
    // Check if we are on a leaf program (program with no other on top of it)
    if parent_to_children.get(current_program).unwrap().is_empty() {
        tower_weights.insert(
            current_program.to_string(),
            *program_weights.get(current_program).unwrap(),
        );
    } else {
        // Record the tower weights mapped to the program weights of child towers
        let mut program_tower_weight = 0;
        let mut weight_occurrences: HashMap<u64, Vec<u64>> = HashMap::new();
        for child in parent_to_children.get(current_program).unwrap() {
            // Only proceed if all child programs have been visited
            if !tower_weights.contains_key(child) {
                return None;
            }
            let child_weight = *tower_weights.get(child).unwrap();
            program_tower_weight += child_weight;
            if let Entry::Vacant(e) = weight_occurrences.entry(child_weight) {
                e.insert(vec![*program_weights.get(child).unwrap()]);
            } else {
                weight_occurrences
                    .get_mut(&child_weight)
                    .unwrap()
                    .push(*program_weights.get(child).unwrap());
            }
        }
        // Check if mismatched tower weight is found
        let tower_weight_balanced = *weight_occurrences
            .iter()
            .max_by_key(|entry| entry.1.len())
            .unwrap()
            .0;
        let (&tower_weight_unbalanced, &program_weight_unbalanced) = weight_occurrences
            .iter()
            .min_by_key(|entry| entry.1.len())
            .map(|(k, v)| (k, v.iter().next().unwrap()))
            .unwrap();
        if tower_weight_balanced != tower_weight_unbalanced {
            let delta_weight = u64::abs_diff(tower_weight_balanced, tower_weight_unbalanced);
            let program_weight_corrected = if tower_weight_balanced > tower_weight_unbalanced {
                program_weight_unbalanced + delta_weight
            } else {
                program_weight_unbalanced - delta_weight
            };
            return Some(program_weight_corrected);
        }
        // Record the tower weight for current program
        program_tower_weight += program_weights.get(current_program).unwrap();
        tower_weights.insert(current_program.to_string(), program_tower_weight);
    }
    // Proceed to the parent of the current program
    if let Some(parent) = child_to_parent.get(current_program) {
        return find_unbalanced_program_corrected_weight_recursive(
            parent,
            program_weights,
            parent_to_children,
            child_to_parent,
            tower_weights,
        );
    }
    // Reached the bottom program without finding an unbalanced tower weight
    None
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
