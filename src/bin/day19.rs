use core::fmt;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

use aoc_utils::cartography::{CardinalDirection, Point2D};

const PROBLEM_NAME: &str = "A Series of Tubes";
const PROBLEM_INPUT_FILE: &str = "./input/day19.txt";
const PROBLEM_DAY: u64 = 19;

/// Represents the unique variants of track segments in the Day 19 problem input file.
#[derive(Copy, Clone, PartialEq, Eq)]
enum TrackSegment {
    Vertical,
    Horizontal,
    Corner,
    Letter { letter: char },
}

/// Custom error type indicating that the operation parsing the input file has failed.
#[derive(Debug)]
struct InputFileParseError {
    message: String,
}

impl fmt::Display for InputFileParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parsing of input file failed: {}", self.message)
    }
}

/// Custom error type indicating an unrecoverable error has been encountered in navigating the track
/// segments.
#[derive(Debug)]
struct NavigationError {
    message: String,
}

impl fmt::Display for NavigationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Navigation error encountered: {}", self.message)
    }
}

/// Processes the AOC 2017 Day 19 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 19 input file in the format required by the solver functions.
///
/// Returned value is [`HashMap`] mapping a two-dimensional location ([`Point2D`]) to a track
/// segment type ([`TrackSegment`]).
fn process_input_file(filename: &str) -> HashMap<Point2D, TrackSegment> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    parse_input_file_contents(&raw_input).unwrap()
}

/// Parses the content of the input file to generate the data structure needed as input to the
/// problem solver functions.
///
/// If the input file is correctly formatted, a [`HashMap`] is returned. Otherwise, an
/// [`InputFileParseError`] is returned to indicate the parsing operation has failed.
fn parse_input_file_contents(
    s: &str,
) -> Result<HashMap<Point2D, TrackSegment>, InputFileParseError> {
    let mut track_map: HashMap<Point2D, TrackSegment> = HashMap::new();
    let mut max_col: Option<usize> = None;
    for (r, row) in s.lines().enumerate() {
        for (c, tile) in row.chars().enumerate() {
            let (x, y) = (c as i64, r as i64);
            match tile {
                '|' => {
                    track_map.insert(Point2D::new(x, y), TrackSegment::Vertical);
                }
                '-' => {
                    track_map.insert(Point2D::new(x, y), TrackSegment::Horizontal);
                }
                '+' => {
                    track_map.insert(Point2D::new(x, y), TrackSegment::Corner);
                }
                'A'..='Z' => {
                    track_map.insert(Point2D::new(x, y), TrackSegment::Letter { letter: tile });
                }
                _ => (),
            }
            // Check that the current row is not too long!
            match r {
                0 => max_col = Some(c),
                _ => {
                    if c > max_col.unwrap() {
                        return Err(InputFileParseError {
                            message: format!("Row {r} is too long!"),
                        });
                    }
                }
            }
        }
    }
    Ok(track_map)
}

/// Solves AOC 2017 Day 19 Part 1.
///
/// Determines the sequence of letters encountered by the packet as it navigates the track segments
/// given in the input [`HashMap`].
fn solve_part1(track_map: &HashMap<Point2D, TrackSegment>) -> String {
    let (letters, _) = navigate_track(track_map);
    letters
}

/// Solves AOC 2017 Day 19 Part 2.
///
/// Determines the number of steps required for the packet to complete its navigation through the
/// track segments.
fn solve_part2(track_map: &HashMap<Point2D, TrackSegment>) -> usize {
    let (_, steps) = navigate_track(track_map);
    steps
}

/// Navigates the packet through the track, collecting letters and counting steps along the way.
///
/// Returned tuple contains the letters (in order) collected along the way and the total number of
/// steps undertaken.
fn navigate_track(track_map: &HashMap<Point2D, TrackSegment>) -> (String, usize) {
    // Establish start location
    let mut dirn = CardinalDirection::South;
    let mut loc = *track_map.keys().find(|loc| loc.y() == 0).unwrap();
    let mut letters = String::new();
    // Packet takes a step to enter the starting location
    let mut steps = 1;
    // Continue moving until there is no valid move next
    loop {
        // Move in current direction
        let old_loc = loc;
        let (dx, dy) = calculate_direction_unit_vector(&dirn);
        loc.shift(dx, dy);
        // Check if the final location was reached
        if !track_map.contains_key(&loc) {
            break;
        }
        steps += 1;
        // Check for follow-up actions
        let track_segment = *track_map.get(&loc).unwrap();
        match track_segment {
            TrackSegment::Letter { letter } => letters.push(letter),
            TrackSegment::Corner => {
                dirn =
                    determine_new_direction_from_corner(track_map, &loc, &old_loc, &dirn).unwrap()
            }
            _ => (),
        }
    }
    (letters, steps)
}

/// Gets the new location for the packet that has moved into a corner segment.
///
/// Returns None if the corner has only one or fewer track segments leading into it.
fn get_next_location_from_corner(
    track_map: &HashMap<Point2D, TrackSegment>,
    loc: &Point2D,
    old_loc: &Point2D,
) -> Option<Point2D> {
    let adjacent_points = loc.get_adjacent_points();
    for loc in adjacent_points {
        if loc == *old_loc {
            continue;
        }
        if track_map.contains_key(&loc) {
            return Some(loc);
        }
    }
    None
}

/// Determines the new direction of the packet after it enters a corner track segment.
fn determine_new_direction_from_corner(
    track_map: &HashMap<Point2D, TrackSegment>,
    loc: &Point2D,
    old_loc: &Point2D,
    dirn: &CardinalDirection,
) -> Result<CardinalDirection, NavigationError> {
    // Determine the next location
    let next_loc = get_next_location_from_corner(track_map, loc, old_loc).unwrap();
    // Calculate the corrected unit vector for the new direction
    let (dx, dy) = calculate_direction_unit_vector(dirn);
    let (ddx, ddy) = (
        next_loc.x() - old_loc.x() - dx,
        next_loc.y() - old_loc.y() - dy,
    );
    // Match the corrected unit vector to the new direction
    match (ddx, ddy) {
        (-1, 0) => Ok(CardinalDirection::West),
        (1, 0) => Ok(CardinalDirection::East),
        (0, -1) => Ok(CardinalDirection::North),
        (0, 1) => Ok(CardinalDirection::South),
        _ => Err(NavigationError {
            message: String::from("Failed to determine new direction from corner segment!"),
        }),
    }
}

/// Calculates the unit vector for the given [`CardinalDirection`] variant.
fn calculate_direction_unit_vector(dirn: &CardinalDirection) -> (i64, i64) {
    match dirn {
        CardinalDirection::North => (0, -1),
        CardinalDirection::East => (1, 0),
        CardinalDirection::South => (0, 1),
        CardinalDirection::West => (-1, 0),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 19 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day19_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!("QPRYCIOLU", solution);
    }

    /// Tests the Day 19 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day19_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(16162, solution);
    }
}
