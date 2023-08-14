use std::time::Instant;
use std::{fs, str::FromStr};

use aoc2017::utils::machines::soundcomputer::{Instruction, SoundComputer};

const PROBLEM_NAME: &str = "Duet";
const PROBLEM_INPUT_FILE: &str = "./input/day18.txt";
const PROBLEM_DAY: u64 = 18;

/// Processes the AOC 2017 Day 18 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 18 input file in the format required by the solver functions.
///
/// Returned value is vector of [`Instruction`] given by the lines of the input file.
fn process_input_file(filename: &str) -> Vec<Instruction> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect::<Vec<Instruction>>()
}

/// Solves AOC 2017 Day 18 Part 1.
///
/// Determines the value of the recovered frequency the first time a "rcv" instruction is executed
/// with a non-zero value.
fn solve_part1(instructions: &[Instruction]) -> i64 {
    let mut sound_computer = SoundComputer::new(instructions, false);
    sound_computer.execute();
    sound_computer.get_last_sent_sound().unwrap()
}

/// Solves AOC 2017 Day 18 Part 2.
///
/// ###
fn solve_part2(_instructions: &[Instruction]) -> u64 {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 18 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day18_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(3188, solution);
    }

    /// Tests the Day 18 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day18_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(7112, solution);
    }
}
