use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use aoc_utils::cartography::Point2D;

const PROBLEM_NAME: &str = "Spiral Memory";
const PROBLEM_INPUT_FILE: &str = "./input/day03.txt";
const PROBLEM_DAY: u64 = 3;

/// Processes the AOC 2017 Day 03 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 03 input file in the format required by the solver functions.
///
/// Returned value is value given in the input file.
fn process_input_file(filename: &str) -> u64 {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().parse::<u64>().unwrap()
}

/// Solves AOC 2017 Day 03 Part 1.
///
/// Determines the number of steps needed to carry the data from the target square to the access
/// port in the centre of the simple spiral.
fn solve_part1(target: &u64) -> u64 {
    let (_value, loc) = generate_simple_spiral(*target);
    loc.get_manhattan_distance(&Point2D::new(0, 0))
}

/// Solves AOC 2017 Day 03 Part 2.
///
/// Determines the first value over the target value that is generated in the complex spiral.
fn solve_part2(target: &u64) -> u64 {
    let (value, _loc) = generate_complex_spiral(*target);
    value
}

/// Generates a simple spiral and returns the first value over the given target value and its
/// location.
fn generate_simple_spiral(target: u64) -> (u64, Point2D) {
    let mut value = 1;
    let mut loc = Point2D::new(0, 0);
    let mut ring = 0;
    let mut delta = (1, 0);
    while value < target {
        if value == u64::pow(2 * ring + 1, 2) {
            // bottom right
            proceed_to_next_ring_simple_spiral(&mut ring, &mut loc, &mut delta, &mut value);
            continue;
        } else if value == u64::pow(2 * ring + 1, 2) - ring * 2 {
            // bottom left
            delta = (1, 0);
        } else if value == u64::pow(2 * ring + 1, 2) - ring * 4 {
            // top left
            delta = (0, 1);
        } else if value == u64::pow(2 * ring + 1, 2) - ring * 6 {
            // top right
            delta = (-1, 0);
        }
        loc.shift(delta.0, delta.1);
        value += 1;
    }
    (value, loc)
}

/// Updates the spiral parameters to proceed to the next ring going outwards.
fn proceed_to_next_ring_simple_spiral(
    ring: &mut u64,
    loc: &mut Point2D,
    delta: &mut (i64, i64),
    value: &mut u64,
) {
    *ring += 1;
    loc.shift(delta.0, delta.1);
    *delta = (0, -1);
    *value += 1;
}

/// Generates a complex spiral and returns the first value over the given target value and its
/// location.
fn generate_complex_spiral(target: u64) -> (u64, Point2D) {
    let mut value = 1;
    let mut loc = Point2D::new(0, 0);
    let mut ring = 0;
    let mut delta = (1, 0);
    let mut spiral: HashMap<Point2D, u64> = HashMap::new();
    while value < target {
        spiral.insert(loc, value);
        if loc.x() == ring && loc.y() == ring {
            // bottom right
            proceed_to_next_ring_complex_spiral(
                &mut ring, &mut loc, &mut delta, &mut value, &spiral,
            );
            continue;
        } else if loc.x() == ring && loc.y() == -ring {
            // top right
            delta = (-1, 0);
        } else if loc.x() == -ring && loc.y() == -ring {
            // top left
            delta = (0, 1);
        } else if loc.x() == -ring && loc.y() == ring {
            // bottom left
            delta = (1, 0);
        }
        loc.shift(delta.0, delta.1);
        value = calculate_location_value_complex_spiral(&loc, &spiral);
    }
    (value, loc)
}

/// Updates the spiral parameters to proceed to the next ring in a complex spiral.
fn proceed_to_next_ring_complex_spiral(
    ring: &mut i64,
    loc: &mut Point2D,
    delta: &mut (i64, i64),
    value: &mut u64,
    spiral: &HashMap<Point2D, u64>,
) {
    *ring += 1;
    loc.shift(delta.0, delta.1);
    *delta = (0, -1);
    *value = calculate_location_value_complex_spiral(loc, spiral);
}

/// Calculates the value for the given location in the complex spiral, as the sum of all surrounding
/// values existing in the spiral.
fn calculate_location_value_complex_spiral(loc: &Point2D, spiral: &HashMap<Point2D, u64>) -> u64 {
    loc.get_surrounding_points()
        .iter()
        .filter_map(|sloc| spiral.get(sloc))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 03 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day03_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(480, solution);
    }

    /// Tests the Day 03 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day03_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(349975, solution);
    }
}
