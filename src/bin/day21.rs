use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;

use aoc2017::utils::error::InputFileParseError;

const PROBLEM_NAME: &str = "Fractal Art";
const PROBLEM_INPUT_FILE: &str = "./input/day21.txt";
const PROBLEM_DAY: u64 = 21;

lazy_static! {
    /// Regex for matching rule converting a 2x2 grid section into 3x3 grid section
    static ref REGEX_RULE_FOUR: Regex =
        Regex::new(r"^([.#]{2}/[.#]{2}) => ([.#]{3}/[.#]{3}/[.#]{3})$").unwrap();

    /// Regex for matching rule converting 3x3 grid section into 4x4 grid section
    static ref REGEX_RULE_NINE: Regex =
        Regex::new(r"^([.#]{3}/[.#]{3}/[.#]{3}) => ([.#]{4}/[.#]{4}/[.#]{4}/[.#]{4})$").unwrap();

    /// Artgrid state at the start of the problem before any enhancement rules are applied.
    static ref ARTGRID_START: Vec<Vec<char>> = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];
}

/// Processes the AOC 2017 Day 21 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 21 input file in the format required by the solver functions.
///
/// Returned value is HashMap containing the entries representing each of the enhancement rules
/// given in the input file.
fn process_input_file(filename: &str) -> HashMap<String, String> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .lines()
        .map(|line| parse_input_file_line(line).unwrap())
        .collect::<HashMap<String, String>>()
}

/// Parses a single line from the input file to extract the left and right sides of the rule
/// expressions. If line is not a valid format, an [`InputFileParseError`] is returned.
fn parse_input_file_line(s: &str) -> Result<(String, String), InputFileParseError> {
    if let Ok(Some(caps)) = REGEX_RULE_FOUR.captures(s) {
        let left = caps[1].replace('/', "");
        let right = caps[2].replace('/', "");
        return Ok((left, right));
    } else if let Ok(Some(caps)) = REGEX_RULE_NINE.captures(s) {
        let left = caps[1].replace('/', "");
        let right = caps[2].replace('/', "");
        return Ok((left, right));
    }
    Err(InputFileParseError {
        message: format!("Invalid input line format: {}", s),
    })
}

/// Solves AOC 2017 Day 21 Part 1.
///
/// Determines how many pixels are left on after applying 5 iterations of the enhancement rules.
fn solve_part1(rules: &HashMap<String, String>) -> usize {
    let artgrid: Vec<Vec<char>> = generate_art(rules, 5);
    artgrid
        .iter()
        .map(|row| row.iter().filter(|&c| *c == '#').count())
        .sum()
}

/// Solves AOC 2017 Day 21 Part 2.
///
/// Determines how many pixels are left on after applying 18 iterations of the enhancement rules.
fn solve_part2(rules: &HashMap<String, String>) -> usize {
    let artgrid: Vec<Vec<char>> = generate_art(rules, 18);
    artgrid
        .iter()
        .map(|row| row.iter().filter(|&c| *c == '#').count())
        .sum()
}

/// Generates a new art grid by applying n iterations of the enhancement rules over the default
/// art grid.
fn generate_art(rules: &HashMap<String, String>, iterations: usize) -> Vec<Vec<char>> {
    let mut artgrid: Vec<Vec<char>> = vec![
        vec!['.', '#', '.'],
        vec!['.', '.', '#'],
        vec!['#', '#', '#'],
    ];
    for _ in 0..iterations {
        artgrid = apply_enhancement_rules(rules, &artgrid);
    }
    artgrid
}

/// Applies the enhancement rules to the artgrid, returning the new and enhanced artgrid.
fn apply_enhancement_rules(
    rules: &HashMap<String, String>,
    artgrid: &[Vec<char>],
) -> Vec<Vec<char>> {
    // Calculate old and new subgrid units
    let (old_subgrid_unit, new_subgrid_unit) = {
        if artgrid.len() % 2 == 0 {
            (2, 3)
        } else {
            (3, 4)
        }
    };
    // Initialise the new artgrid
    let new_artgrid_size = (artgrid.len() / old_subgrid_unit) * new_subgrid_unit;
    let mut new_artgrid = vec![vec!['.'; new_artgrid_size]; new_artgrid_size];
    // Iterate over the subgrids in the old artgrid
    for r in (0..artgrid.len()).step_by(old_subgrid_unit) {
        'inner: for c in (0..artgrid[r].len()).step_by(old_subgrid_unit) {
            // Extract subgrid
            let mut subgrid = vec![vec!['.'; old_subgrid_unit]; old_subgrid_unit];
            for y in 0..old_subgrid_unit {
                for x in 0..old_subgrid_unit {
                    subgrid[y][x] = artgrid[r + y][c + x];
                }
            }
            // Look for rule match
            for i in 0..8 {
                // Transform subgrid
                subgrid = {
                    if i % 2 == 0 {
                        rot180_artgrid(&subgrid)
                    } else {
                        flip_artgrid(&subgrid)
                    }
                };
                // Convert subgrid to string key
                let s_subgrid = subgrid
                    .iter()
                    .map(|row| row.iter().collect::<String>())
                    .join("");
                // Check for rule match
                if rules.contains_key(&s_subgrid) {
                    let enhanced_subgrid = rules.get(&s_subgrid).unwrap();
                    for (i, elem) in enhanced_subgrid.chars().enumerate() {
                        let delta_r = i / new_subgrid_unit;
                        let delta_c = i % new_subgrid_unit;
                        let r_enhanced = (r / old_subgrid_unit) * new_subgrid_unit + delta_r;
                        let c_enhanced = (c / old_subgrid_unit) * new_subgrid_unit + delta_c;
                        new_artgrid[r_enhanced][c_enhanced] = elem;
                    }
                    continue 'inner;
                }
            }
            panic!("Could not find rule match for subgrid!");
        }
    }
    new_artgrid
}

/// Flips the square artgrid about its centre horizontal axis by inverting the y-axis (rows),
/// leaving columns unchanged.
fn flip_artgrid(artgrid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_artgrid = vec![vec!['.'; artgrid.len()]; artgrid.len()];
    for (r, row) in new_artgrid.iter_mut().enumerate() {
        let old_r = artgrid.len() - r - 1;
        row.copy_from_slice(&artgrid[old_r]);
    }
    new_artgrid
}

/// Rotates the square artgrid by an equivalent of 180 degrees. The operation results in the x-axis
/// (columns) and y-axis (rows) being inverted and switched.
fn rot180_artgrid(artgrid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut new_artgrid = vec![vec!['.'; artgrid.len()]; artgrid.len()];
    for (r, row) in new_artgrid.iter_mut().enumerate() {
        for c in 0..row.len() {
            let old_r = artgrid.len() - r - 1;
            let old_c = row.len() - c - 1;
            row[c] = artgrid[old_c][old_r];
        }
    }
    new_artgrid
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 21 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day21_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(203, solution);
    }

    /// Tests the Day 21 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day21_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(3342470, solution);
    }
}
