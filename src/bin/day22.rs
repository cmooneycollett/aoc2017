use std::time::Instant;
use std::{collections::HashMap, fs};

use aoc_utils::cartography::{CardinalDirection, Point2D};

const PROBLEM_NAME: &str = "Sporifica Virus";
const PROBLEM_INPUT_FILE: &str = "./input/day22.txt";
const PROBLEM_DAY: u64 = 22;

const PART1_BURSTS: usize = 10_000;
const PART2_BURSTS: usize = 10_000_000;

/// Used to represent the possible states of individual grid tile.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum NodeState {
    Clean,
    Infected,
    Weakened,
    Flagged,
}

impl NodeState {
    /// Gets the next state for a node based on its current state.
    fn next_node_state(&self, is_evolved_virus: bool) -> NodeState {
        match self {
            NodeState::Clean => {
                if !is_evolved_virus {
                    NodeState::Infected
                } else {
                    NodeState::Weakened
                }
            }
            NodeState::Infected => {
                if !is_evolved_virus {
                    NodeState::Clean
                } else {
                    NodeState::Flagged
                }
            }
            NodeState::Weakened => NodeState::Infected,
            NodeState::Flagged => NodeState::Clean,
        }
    }
}

/// Custom type representing the input to the problem solver functions. The tuple value contains the
/// starting state of the computer grid, and the maximum x- and y-coordinates for the tiles in the
/// grid.
type ProblemInput = (HashMap<Point2D, NodeState>, i64, i64);

/// Processes the AOC 2017 Day 22 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 22 input file in the format required by the solver functions.
///
/// Returned value is tuple containing the initial grid state given in the input file and the
/// maximum x- and y-coordinates of grid locations. The top left tile given in the input file is
/// taken to have the location (x,y):(0,0).
fn process_input_file(filename: &str) -> ProblemInput {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut grid_state: HashMap<Point2D, NodeState> = HashMap::new();
    let mut max_x: Option<usize> = None;
    let mut max_y: Option<usize> = None;
    for (y, row) in raw_input.lines().enumerate() {
        // Track length of the current row
        let mut row_max_x: Option<usize> = None;
        for (x, tile) in row.trim().chars().enumerate() {
            row_max_x = Some(x);
            let loc = Point2D::new(i64::try_from(x).unwrap(), i64::try_from(y).unwrap());
            match tile {
                '.' => {
                    grid_state.insert(loc, NodeState::Clean);
                }
                '#' => {
                    grid_state.insert(loc, NodeState::Infected);
                }
                _c => panic!("Invalid character in input file at ({x},{y}): {_c}"),
            }
        }
        // Check if the row was empty
        if row_max_x.is_none() {
            panic!("Empty row at row {y}!");
        }
        // Update expected row length and check that current row was not too long or short
        if max_x.is_none() {
            max_x = Some(row_max_x.unwrap());
        } else if row_max_x.unwrap() != max_x.unwrap() {
            panic!("Row {y} is not the same length as preceding rows in input file!");
        }
        // Update maximum observed y-coordinate
        max_y = Some(y);
    }
    // Check that a maximum x- and y-coordinate have been found
    if max_x.is_none() || max_y.is_none() {
        panic!("Malformed input file - empty rows!");
    }
    (
        grid_state,
        i64::try_from(max_x.unwrap()).unwrap(),
        i64::try_from(max_y.unwrap()).unwrap(),
    )
}

/// Solves AOC 2017 Day 22 Part 1.
///
/// Determines how many bursts of activity cause a node to become infected after 10,000 bursts of
/// activity.
fn solve_part1(input: &ProblemInput) -> usize {
    let (grid, max_x, max_y) = input;
    conduct_bursts(grid, *max_x, *max_y, PART1_BURSTS, false)
}

/// Solves AOC 2017 Day 22 Part 2.
///
/// Determines how many bursts of activity cause a node to become infect after 10,000,000 bursts of
/// activity using an evolved virus.
fn solve_part2(input: &ProblemInput) -> usize {
    let (grid, max_x, max_y) = input;
    conduct_bursts(grid, *max_x, *max_y, PART2_BURSTS, true)
}

/// Determines the number of bursts of activity that cause a node to become infected.
fn conduct_bursts(
    grid: &HashMap<Point2D, NodeState>,
    max_x: i64,
    max_y: i64,
    num_bursts: usize,
    is_evolved_virus: bool,
) -> usize {
    let mut grid = grid.clone();
    // Initialise carrier location and direction
    let start_x = max_x / 2 + max_x % 2;
    let start_y = max_y / 2 + max_y % 2;
    let mut loc_carrier = Point2D::new(start_x, start_y);
    let mut dirn_carrier: CardinalDirection = CardinalDirection::North;
    let mut infection_bursts: usize = 0;
    for _ in 0..num_bursts {
        // Add surrounding locations to grid as clean nodes (if not already present)
        add_surrounding_nodes_to_grid(&mut grid, &loc_carrier);
        // Update carrier direction
        let node_state = grid.get(&loc_carrier).unwrap();
        dirn_carrier = match node_state {
            NodeState::Clean => dirn_carrier.rotate90_counterclockwise(1),
            NodeState::Infected => dirn_carrier.rotate90_clockwise(1),
            NodeState::Weakened => dirn_carrier,
            NodeState::Flagged => dirn_carrier.rotate90_clockwise(2),
        };
        // Update node state and check new state to count infection bursts
        grid.insert(loc_carrier, node_state.next_node_state(is_evolved_virus));
        if *grid.get(&loc_carrier).unwrap() == NodeState::Infected {
            infection_bursts += 1;
        }
        // Update carrier location
        match dirn_carrier {
            CardinalDirection::North => loc_carrier.shift(0, -1),
            CardinalDirection::East => loc_carrier.shift(1, 0),
            CardinalDirection::South => loc_carrier.shift(0, 1),
            CardinalDirection::West => loc_carrier.shift(-1, 0),
        }
    }
    infection_bursts
}

/// Adds clean nodes to the grid around the given location if they are not already recorded in the
/// grid.
fn add_surrounding_nodes_to_grid(grid: &mut HashMap<Point2D, NodeState>, loc: &Point2D) {
    for loc_surround in loc.get_surrounding_points() {
        grid.entry(loc_surround).or_insert(NodeState::Clean);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 22 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day22_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(5570, solution);
    }

    /// Tests the Day 22 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day22_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(2512022, solution);
    }
}
