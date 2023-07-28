use std::fs;
use std::time::Instant;

use aoc_utils::cartography::Point3D;

const PROBLEM_NAME: &str = "Hex Ed";
const PROBLEM_INPUT_FILE: &str = "./input/day11.txt";
const PROBLEM_DAY: u64 = 11;

/// Represents the six virtual directions from one hexagon tile to another adjoining tile.
enum HexGridDirection {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl HexGridDirection {
    /// Converts the given string to the corresponding variant of [`HexGridDirection`].
    fn from_string(s: &str) -> Option<HexGridDirection> {
        match s.trim().to_lowercase().as_str() {
            "n" => Some(HexGridDirection::North),
            "ne" => Some(HexGridDirection::NorthEast),
            "se" => Some(HexGridDirection::SouthEast),
            "s" => Some(HexGridDirection::South),
            "sw" => Some(HexGridDirection::SouthWest),
            "nw" => Some(HexGridDirection::NorthWest),
            _ => None,
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
        .split(",")
        .filter_map(|s| HexGridDirection::from_string(s))
        .collect::<Vec<HexGridDirection>>()
}

/// Solves AOC 2017 Day 11 Part 1 // Determine the number of steps required to reach the final
/// location after processing all of the hexagon grid moves.
fn solve_part1(input: &[HexGridDirection]) -> u64 {
    let mut loc = Point3D::new(0, 0, 0);
    for dirn in input {
        update_point3d_location(&mut loc, dirn);
    }
    let coords = [loc.x().abs(), loc.y().abs(), loc.z().abs()];
    coords.iter().max().unwrap().unsigned_abs()
}

/// Solves AOC 2017 Day 11 Part 2 // ###
fn solve_part2(_input: &[HexGridDirection]) -> u64 {
    0
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
