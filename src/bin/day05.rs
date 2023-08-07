use std::fs;
use std::time::Instant;

const PROBLEM_NAME: &str = "A Maze of Twisty Trampolines, All Alike";
const PROBLEM_INPUT_FILE: &str = "./input/day05.txt";
const PROBLEM_DAY: u64 = 5;

/// Processes the AOC 2017 Day 05 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 05 input file in the format required by the solver functions.
///
/// Returned value is vector of integer values given in the lines of the input file.
fn process_input_file(filename: &str) -> Vec<isize> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

/// Solves AOC 2017 Day 05 Part 1.
///
/// Determines the number of steps needed for the cursor to exit the jump space.
fn solve_part1(jumps: &[isize]) -> u64 {
    calculate_steps_to_exit_jumpspace(jumps, false)
}

/// Solves AOC 2017 Day 05 Part 2.
///
/// Determines the number of steps needed for the cursor to exit the jump space, using strange
/// jumps.
fn solve_part2(jumps: &[isize]) -> u64 {
    calculate_steps_to_exit_jumpspace(jumps, true)
}

/// Calculates the number of steps needed for the cursor to exit the jump space.
///
/// If strange jumps are used, the location value that is being jumped from by the cursor is
/// decreased by 1 if the offset was 3 or more; otherwise (or if not using strange jumps), the
/// location value is increased by 1.
fn calculate_steps_to_exit_jumpspace(jumps: &[isize], strange_jumps: bool) -> u64 {
    // Check if the jump space is empty
    if jumps.is_empty() {
        return 0;
    }
    // Initialise
    let mut jumps = jumps.to_vec();
    let mut cursor = 0;
    let mut steps = 0;
    loop {
        let delta = jumps[cursor];
        // Update the location value being jumped from by the cursor
        jumps[cursor] += {
            if delta >= 3 && strange_jumps {
                -1
            } else {
                1
            }
        };
        steps += 1;
        // Check if jump takes cursor outside of jump space, and update cursor location
        match delta.is_positive() {
            true => {
                if delta.unsigned_abs() + cursor >= jumps.len() {
                    break;
                }
                cursor += delta.unsigned_abs();
            }
            false => {
                if delta.unsigned_abs() > cursor {
                    break;
                }
                cursor -= delta.unsigned_abs();
            }
        }
    }
    steps
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 05 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day05_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(358131, solution);
    }

    /// Tests the Day 05 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day05_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(25558839, solution);
    }
}
