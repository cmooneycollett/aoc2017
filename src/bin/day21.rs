use std::collections::HashMap;
use std::fs;
use std::iter;
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

    /// Sequence of transformations required to check all eight members of the symmetry group for a
    /// 2x2 array.
    static ref TRANSFORMATION_GRID2: [fn(Box<[[char; 2]; 2]>) -> Box<[[char; 2]; 2]>; 7] =
        [flip_ud_grid2, flip_lr_grid2, rot90_ccw_grid2, flip_lr_grid2, flip_ud_grid2, flip_lr_grid2,
        rot90_ccw_grid2];

    /// Sequence of transformations required to check all eight members of the symmetry group for a
    /// 3x3 array.
    static ref TRANSFORMATION_GRID3: [fn(Box<[[char; 3]; 3]>) -> Box<[[char; 3]; 3]>; 7] =
        [flip_ud_grid3, flip_lr_grid3, rot90_ccw_grid3, flip_lr_grid3, flip_ud_grid3, flip_lr_grid3,
        rot90_ccw_grid3];

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
/// ###
fn solve_part1(rules: &HashMap<String, String>) -> usize {
    let artgrid: Vec<Vec<char>> = generate_art(rules, 5);
    artgrid
        .iter()
        .map(|row| row.iter().filter(|&c| *c == '#').count())
        .sum()
}

/// Solves AOC 2017 Day 21 Part 2.
///
/// ###
fn solve_part2(_input: &HashMap<String, String>) -> usize {
    unimplemented!();
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
        artgrid = {
            if artgrid.len() % 2 == 0 {
                apply_enhancement_rules_grid2(rules, &artgrid)
            } else {
                apply_enhancement_rules_grid3(rules, &artgrid)
            }
        };
    }
    artgrid
}

/// Applies the enhancement rules to the artgrid with size being a multiple of 3 to generate a new
/// and enhanced artgrid.
fn apply_enhancement_rules_grid3(
    rules: &HashMap<String, String>,
    artgrid: &[Vec<char>],
) -> Vec<Vec<char>> {
    // Initialise the new artgrid
    let (old_unit, new_unit) = (3, 4);
    let new_size = (artgrid.len() / old_unit) * new_unit;
    let mut new_artgrid: Vec<Vec<char>> = vec![];
    for _ in 0..new_size {
        new_artgrid.push(iter::repeat('.').take(new_size).collect::<Vec<char>>());
    }
    for r in (0..artgrid.len()).step_by(old_unit) {
        'inner: for c in (0..artgrid.len()).step_by(old_unit) {
            let mut subgrid = Box::new([
                [artgrid[r][c], artgrid[r][c + 1], artgrid[r][c + 2]],
                [
                    artgrid[r + 1][c],
                    artgrid[r + 1][c + 1],
                    artgrid[r + 1][c + 2],
                ],
                [
                    artgrid[r + 2][c],
                    artgrid[r + 2][c + 1],
                    artgrid[r + 2][c + 2],
                ],
            ]);
            // Flip
            for i in 0..8 {
                if i > 0 {
                    subgrid = TRANSFORMATION_GRID3[i - 1](subgrid);
                }
                let s_subgrid = subgrid
                    .iter()
                    .map(|row| row.iter().collect::<String>())
                    .join("");
                if rules.contains_key(&s_subgrid) {
                    let enhanced_subgrid = rules.get(&s_subgrid).unwrap();
                    for (i, elem) in enhanced_subgrid.chars().enumerate() {
                        let delta_r = i / new_unit;
                        let delta_c = i % new_unit;
                        let r_enhanced = (r / old_unit) * new_unit + delta_r;
                        let c_enhanced = (c / old_unit) * new_unit + delta_c;
                        new_artgrid[r_enhanced][c_enhanced] = elem;
                    }
                    continue 'inner;
                }
            }
            panic!("Could not find enhanced subgrid - grid3");
        }
    }
    new_artgrid
}

/// Applies the enhancement rules to the artgrid with size being a multiple of 2 to generate a new
/// and enhanced artgrid.
fn apply_enhancement_rules_grid2(
    rules: &HashMap<String, String>,
    artgrid: &[Vec<char>],
) -> Vec<Vec<char>> {
    // Initialise the new artgrid
    let (old_unit, new_unit) = (2, 3);
    let new_size = (artgrid.len() / old_unit) * new_unit;
    let mut new_artgrid: Vec<Vec<char>> = vec![];
    for _ in 0..new_size {
        new_artgrid.push(iter::repeat('.').take(new_size).collect::<Vec<char>>());
    }
    for r in (0..artgrid.len()).step_by(old_unit) {
        'inner: for c in (0..artgrid.len()).step_by(old_unit) {
            let mut subgrid = Box::new([
                [artgrid[r][c], artgrid[r][c + 1]],
                [artgrid[r + 1][c], artgrid[r + 1][c + 1]],
            ]);
            // Flip
            for i in 0..8 {
                if i > 0 {
                    subgrid = TRANSFORMATION_GRID2[i - 1](subgrid);
                }
                let s_subgrid = subgrid
                    .iter()
                    .map(|row| row.iter().collect::<String>())
                    .join("");
                if rules.contains_key(&s_subgrid) {
                    let enhanced_subgrid = rules.get(&s_subgrid).unwrap();
                    for (i, elem) in enhanced_subgrid.chars().enumerate() {
                        let delta_r = i / new_unit;
                        let delta_c = i % new_unit;
                        let r_enhanced = (r / old_unit) * new_unit + delta_r;
                        let c_enhanced = (c / old_unit) * new_unit + delta_c;
                        new_artgrid[r_enhanced][c_enhanced] = elem;
                    }
                    continue 'inner;
                }
            }
            panic!("Could not find enhanced subgrid - grid3");
        }
    }
    new_artgrid
}

/// Flips a 2x2 array about its horizontal axis of symmetry (up/down flip).
fn flip_ud_grid2(input: Box<[[char; 2]; 2]>) -> Box<[[char; 2]; 2]> {
    Box::new([[input[1][0], input[1][1]], [input[0][0], input[0][1]]])
}

/// Flips a 2x2 array about its vertical axis of symmetry (left/right flip).
fn flip_lr_grid2(input: Box<[[char; 2]; 2]>) -> Box<[[char; 2]; 2]> {
    Box::new([[input[0][1], input[0][0]], [input[1][1], input[1][0]]])
}

/// Rotates a 2x2 array by 90 degrees counterclockwise.
fn rot90_ccw_grid2(input: Box<[[char; 2]; 2]>) -> Box<[[char; 2]; 2]> {
    Box::new([[input[0][1], input[1][1]], [input[0][0], input[1][0]]])
}

/// Flips a 3x3 array about its horizontal axis of symmetry (up/down flip).
fn flip_ud_grid3(input: Box<[[char; 3]; 3]>) -> Box<[[char; 3]; 3]> {
    Box::new([
        [input[2][0], input[2][1], input[2][2]],
        [input[1][0], input[1][1], input[1][2]],
        [input[0][0], input[0][1], input[0][2]],
    ])
}

/// Flips a 3x3 array about its vertical axis of symmetry (left/right flip).
fn flip_lr_grid3(input: Box<[[char; 3]; 3]>) -> Box<[[char; 3]; 3]> {
    Box::new([
        [input[0][2], input[0][1], input[0][0]],
        [input[1][2], input[1][1], input[1][0]],
        [input[2][2], input[2][1], input[2][0]],
    ])
}

/// Rotates a 3x3 array by 90 degrees counterclockwise.
fn rot90_ccw_grid3(input: Box<[[char; 3]; 3]>) -> Box<[[char; 3]; 3]> {
    Box::new([
        [input[0][2], input[1][2], input[2][2]],
        [input[0][1], input[1][1], input[2][1]],
        [input[0][0], input[1][0], input[2][0]],
    ])
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
