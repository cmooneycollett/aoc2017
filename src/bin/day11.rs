use std::fs;
use std::str::FromStr;
use std::time::Instant;

use aoc_utils::cartography::Point3D;

const PROBLEM_NAME: &str = "Hex Ed";
const PROBLEM_INPUT_FILE: &str = "./input/day11.txt";
const PROBLEM_DAY: u64 = 11;

/// Custom error type indicating that the parsing of an HexGridDirection has failed.
#[derive(Debug)]
struct HexGridDirectionParseError;

/// Represents the six virtual directions from one hexagon tile to another adjoining tile.
enum HexGridDirection {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl FromStr for HexGridDirection {
    type Err = HexGridDirectionParseError;

    /// Converts the given string to the corresponding variant of [`HexGridDirection`].
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "n" => Ok(HexGridDirection::North),
            "ne" => Ok(HexGridDirection::NorthEast),
            "se" => Ok(HexGridDirection::SouthEast),
            "s" => Ok(HexGridDirection::South),
            "sw" => Ok(HexGridDirection::SouthWest),
            "nw" => Ok(HexGridDirection::NorthWest),
            _ => Err(HexGridDirectionParseError),
        }
    }
}

/// Processes the AOC 2017 Day 11 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 11 input file in the format required by the solver functions.
/// Returned value is vector of [`HexGridDirection`] variants based on the comma-separated strings
/// extracted from the input file.
fn process_input_file(filename: &str) -> Vec<HexGridDirection> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .split(',')
        .filter_map(|s| HexGridDirection::from_str(s).ok())
        .collect::<Vec<HexGridDirection>>()
}

/// Solves AOC 2017 Day 11 Part 1 // Determine the number of steps required to reach the final
/// location after processing all of the hexagon grid moves.
fn solve_part1(input: &[HexGridDirection]) -> u64 {
    let mut loc = Point3D::new(0, 0, 0);
    for dirn in input {
        update_point3d_location(&mut loc, dirn);
    }
    get_steps_from_origin(&loc)
}

/// Solves AOC 2017 Day 11 Part 2 // Determines the maximum number of steps from the origin that the
/// child process reaches during its journey.
fn solve_part2(input: &[HexGridDirection]) -> u64 {
    let mut loc = Point3D::new(0, 0, 0);
    let mut maximum_distance = 0;
    for dirn in input {
        update_point3d_location(&mut loc, dirn);
        let distance = get_steps_from_origin(&loc);
        if distance > maximum_distance {
            maximum_distance = distance;
        }
    }
    maximum_distance
}

/// Updates the Point3D location based on the next tile specified by the [`HexGridDirection`]
/// variant.
fn update_point3d_location(loc: &mut Point3D, dirn: &HexGridDirection) {
    match dirn {
        HexGridDirection::North => loc.shift(0, -1, 1),
        HexGridDirection::NorthEast => loc.shift(1, -1, 0),
        HexGridDirection::SouthEast => loc.shift(1, 0, -1),
        HexGridDirection::South => loc.shift(0, 1, -1),
        HexGridDirection::SouthWest => loc.shift(-1, 1, 0),
        HexGridDirection::NorthWest => loc.shift(-1, 0, 1),
    }
}

/// Gets the number of steps from the origin represented by the Point3D location. The location is
/// taken to be a point on a two-dimensional hexagon grid.
fn get_steps_from_origin(loc: &Point3D) -> u64 {
    let coords = [loc.x().abs(), loc.y().abs(), loc.z().abs()];
    coords.iter().max().unwrap().unsigned_abs()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 11 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day11_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(877, solution);
    }

    /// Tests the Day 11 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day11_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(1622, solution);
    }
}
