use std::collections::{HashSet, VecDeque};
use std::fs;
use std::num::ParseIntError;
use std::time::Instant;

use aoc2017::utils::knot_hash::calculate_knot_hash;
use itertools::iproduct;

const PROBLEM_NAME: &str = "Disk Defragmentation";
const PROBLEM_INPUT_FILE: &str = "./input/day14.txt";
const PROBLEM_DAY: u64 = 14;

const DISK_GRID_MIN_X: usize = 0;
const DISK_GRID_MAX_X: usize = 127;
const DISK_GRID_MIN_Y: usize = 0;
const DISK_GRID_MAX_Y: usize = 127;

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
///
/// Returned value is string given in the input file.
fn process_input_file(filename: &str) -> String {
    // Read contents of problem input file
    fs::read_to_string(filename).unwrap().trim().to_string()
}

/// Solves AOC 2017 Day 14 Part 1.
///
/// Determines the number of squares used in the disk grid, with rows based on knot hash
/// calculations.
fn solve_part1(input: &str) -> usize {
    (0..=127)
        .map(|v| calculate_knot_hash(&format!("{input}-{v}")))
        .map(|s| convert_string_hexadecimal_to_binary(&s).unwrap())
        .map(|s| s.chars().filter(|c| *c == '1').count())
        .sum()
}

/// Solves AOC 2017 Day 14 Part 2.
///
/// Determines the number of regions present in the disk grid.
fn solve_part2(input: &str) -> usize {
    // Generate disk grid (128x128 grid)
    let disk_grid: Vec<Vec<char>> = (0..=127)
        .map(|v| calculate_knot_hash(&format!("{input}-{v}")))
        .map(|s| {
            convert_string_hexadecimal_to_binary(&s)
                .unwrap()
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    // Set up for the breadth-first search
    let mut region_count = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    // Try each grid location as possible region starting point
    for (x, y) in iproduct!(
        DISK_GRID_MIN_X..=DISK_GRID_MAX_X,
        DISK_GRID_MIN_Y..=DISK_GRID_MAX_Y
    ) {
        // New region found if location is used and not already visited
        if !visited.contains(&(x, y)) && disk_grid[y][x] == '1' {
            let region_locations = determine_region_locations(x, y, &disk_grid);
            visited.extend(region_locations.iter());
            region_count += 1;
        }
    }
    region_count
}

/// Determines the locations in the region containing the starting location.
fn determine_region_locations(
    start_x: usize,
    start_y: usize,
    disk_grid: &[Vec<char>],
) -> HashSet<(usize, usize)> {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut visit_queue: VecDeque<(usize, usize)> = VecDeque::from([(start_x, start_y)]);
    while !visit_queue.is_empty() {
        let (x, y) = visit_queue.pop_front().unwrap();
        visited.insert((x, y));
        for next_loc in determine_possible_next_locations(x, y, disk_grid) {
            if !visited.contains(&next_loc) {
                visit_queue.push_back(next_loc)
            }
        }
    }
    visited
}

/// Determine the possible next locations for a region based on the current x- and y-value.
///
/// This function takes into account the disk grid size when checking for possible next locations.
/// Only locations containing a "used" marker (denoted with a '1') are candidates for possible next
/// locations.
fn determine_possible_next_locations(
    x: usize,
    y: usize,
    disk_grid: &[Vec<char>],
) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = vec![];
    // Check for case where (x,y) is at the top, bottom, left or right edge of grid
    if x > DISK_GRID_MIN_X && disk_grid[y][x - 1] == '1' {
        output.push((x - 1, y));
    }
    if x < DISK_GRID_MAX_X && disk_grid[y][x + 1] == '1' {
        output.push((x + 1, y));
    }
    if y > DISK_GRID_MIN_Y && disk_grid[y - 1][x] == '1' {
        output.push((x, y - 1));
    }
    if y < DISK_GRID_MAX_Y && disk_grid[y + 1][x] == '1' {
        output.push((x, y + 1));
    }
    output
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
