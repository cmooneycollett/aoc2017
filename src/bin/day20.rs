use std::fs;
use std::time::Instant;

use aoc_utils::cartography::Point3D;
use fancy_regex::Regex;
use lazy_static::lazy_static;

use aoc2017::utils::day20::Particle3D;
use aoc2017::utils::error::InputFileParseError;

const PROBLEM_NAME: &str = "Particle Swarm";
const PROBLEM_INPUT_FILE: &str = "./input/day20.txt";
const PROBLEM_DAY: u64 = 20;

lazy_static! {
    static ref REGEX_PARTICLE: Regex = Regex::new(
        r"^p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>$"
    )
    .unwrap();
}

/// Processes the AOC 2017 Day 20 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 20 input file in the format required by the solver functions.
///
/// Returned value is vector of [`Particle3D`] structs created using the values given in the lines
/// of the input file.
fn process_input_file(filename: &str) -> Vec<Particle3D> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .lines()
        .map(|line| parse_input_file_line(line).unwrap())
        .collect::<Vec<Particle3D>>()
}

/// Parses a line from the input file, returning a [`Particle3D`] if the line is correctly
/// formatted. Otherwise, a error ([`InputFileParseError`]) is returned.
fn parse_input_file_line(s: &str) -> Result<Particle3D, InputFileParseError> {
    if let Ok(Some(caps)) = REGEX_PARTICLE.captures(s) {
        // Location
        let loc = {
            let x = caps[1].parse::<i64>().unwrap();
            let y = caps[2].parse::<i64>().unwrap();
            let z = caps[3].parse::<i64>().unwrap();
            Point3D::new(x, y, z)
        };
        // Velocity
        let vel = {
            let x = caps[4].parse::<i64>().unwrap();
            let y = caps[5].parse::<i64>().unwrap();
            let z = caps[6].parse::<i64>().unwrap();
            Point3D::new(x, y, z)
        };
        // Acceleration
        let acc = {
            let x = caps[7].parse::<i64>().unwrap();
            let y = caps[8].parse::<i64>().unwrap();
            let z = caps[9].parse::<i64>().unwrap();
            Point3D::new(x, y, z)
        };
        return Ok(Particle3D::new(&loc, &vel, &acc));
    }
    Err(InputFileParseError {
        message: format!("Input file line not correctly formatted [{s}]"),
    })
}

/// Solves AOC 2017 Day 20 Part 1.
///
/// Returns the number of the particle that will remain closest to the origin in the long-term.
fn solve_part1(particles: &[Particle3D]) -> usize {
    particles
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
        .map(|(index, _)| index)
        .unwrap()
}

/// Solves AOC 2017 Day 20 Part 2.
///
/// ###
fn solve_part2(_input: &[Particle3D]) -> usize {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 20 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day20_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(376, solution);
    }

    /// Tests the Day 20 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day20_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(574, solution);
    }
}
