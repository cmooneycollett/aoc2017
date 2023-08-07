use std::collections::VecDeque;
use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Permutation Promenade";
const PROBLEM_INPUT_FILE: &str = "./input/day16.txt";
const PROBLEM_DAY: u64 = 16;
/// The programs start out in this order at the beginning of each problem part.
const PROGRAM_STARTING_ORDER: &str = "abcdefghijklmnop";

/// Custom error type indicating that the parsing of a line from the input file has failed.
#[derive(Debug)]
struct InputLineParseError;

/// Custom error type indicating that the lookup of a program index has failed.
#[derive(Debug)]
struct ProgramIndexLookupError;

/// Enum representing the different dance moves that can reorder the programs.
#[derive(Copy, Clone)]
enum DanceMove {
    Spin { steps: usize },
    Exchange { a: usize, b: usize },
    Partner { a: char, b: char },
}

lazy_static! {
    static ref SPIN_RX: Regex = Regex::new(r"s(\d+)").unwrap();
    static ref EXCHANGE_RX: Regex = Regex::new(r"x(\d+)/(\d+)").unwrap();
    static ref PARTNER_RX: Regex = Regex::new(r"p([a-p])/([a-p])").unwrap();
}

/// Processes the AOC 2017 Day 16 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 16 input file in the format required by the solver functions.
///
/// Returned value is vector of [`DanceMove`] variants extracted from the comma-separated input
/// file.
fn process_input_file(filename: &str) -> Vec<DanceMove> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    parse_input_file_content(&raw_input).unwrap()
}

/// Solves AOC 2017 Day 16 Part 1.
///
/// Determines the program order after all dance moves have been executed.
fn solve_part1(dance_moves: &[DanceMove]) -> String {
    let mut programs = PROGRAM_STARTING_ORDER.chars().collect::<VecDeque<char>>();
    execute_dance_moves(dance_moves, &mut programs);
    programs.iter().collect::<String>()
}

/// Solves AOC 2017 Day 16 Part 2.
///
/// ###
fn solve_part2(_dance_moves: &[DanceMove]) -> String {
    unimplemented!();
}

/// Parses the content of the input file to generate the data structure needed as input to the
/// problem solver functions.
///
/// If the input file is correctly formatted, a vector of [`DanceMove`] variants is returned.
/// Otherwise, an [`InputLineParseError`] is returned.
fn parse_input_file_content(s: &str) -> Result<Vec<DanceMove>, InputLineParseError> {
    let mut dance_moves: Vec<DanceMove> = vec![];
    for element in s.trim().split(',') {
        let parsed = {
            if let Ok(Some(caps)) = SPIN_RX.captures(element) {
                let steps = caps[1].parse::<usize>().unwrap();
                DanceMove::Spin { steps }
            } else if let Ok(Some(caps)) = EXCHANGE_RX.captures(element) {
                let a = caps[1].parse::<usize>().unwrap();
                let b = caps[2].parse::<usize>().unwrap();
                DanceMove::Exchange { a, b }
            } else if let Ok(Some(caps)) = PARTNER_RX.captures(element) {
                let a = caps[1].parse::<char>().unwrap();
                let b = caps[2].parse::<char>().unwrap();
                DanceMove::Partner { a, b }
            } else {
                return Err(InputLineParseError);
            }
        };
        dance_moves.push(parsed);
    }
    Ok(dance_moves)
}

/// Executes a single round of dance moves, reordering the programs as required.
fn execute_dance_moves(dance_moves: &[DanceMove], programs: &mut VecDeque<char>) {
    for dance in dance_moves {
        match dance {
            DanceMove::Spin { steps } => programs.rotate_right(*steps),
            DanceMove::Exchange { a, b } => programs.swap(*a, *b),
            DanceMove::Partner { a, b } => {
                let (index_a, index_b) = find_program_indices(a, b, programs).unwrap();
                programs.swap(index_a, index_b);
            }
        }
    }
}

/// Finds the index of the A and B programs within the collection of programs.
///
/// If the programs are both in the collection, a tuple containing their respective indices is
/// returned. Otherwise, a [`ProgramIndexLookupError`] is returned.
fn find_program_indices(
    a: &char,
    b: &char,
    programs: &VecDeque<char>,
) -> Result<(usize, usize), ProgramIndexLookupError> {
    let mut index_a: Option<usize> = None;
    let mut index_b: Option<usize> = None;
    for (index, p) in programs.iter().enumerate() {
        // Check if the A or B program is at the current index
        if p == a {
            index_a = Some(index);
        } else if p == b {
            index_b = Some(index);
        }
        // Check if the index of both programs has been found
        if index_a.is_some() && index_b.is_some() {
            return Ok((index_a.unwrap(), index_b.unwrap()));
        }
    }
    Err(ProgramIndexLookupError)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 16 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day16_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!("pkgnhomelfdibjac", solution);
    }

    /// Tests the Day 16 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day16_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!("pogbjfihclkemadn", solution);
    }
}
