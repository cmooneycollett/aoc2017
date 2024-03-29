use std::fs;
use std::time::Instant;

use aoc2017::utils::machines::soundcomputer::{Instruction, SoundComputer};

const PROBLEM_NAME: &str = "Coprocessor Conflagration";
const PROBLEM_INPUT_FILE: &str = "./input/day23.txt";
const PROBLEM_DAY: u64 = 23;

/// Processes the AOC 2017 Day 23 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 23 input file in the format required by the solver functions.
///
/// Returned value is a vector of [`Instruction`] instances given by the lines of the input file.
fn process_input_file(filename: &str) -> Vec<Instruction> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    Instruction::parse_raw_input(&raw_input)
}

/// Solves AOC 2017 Day 23 Part 1.
///
/// Determines the number of times the MUL (multiply) instruction is executed by the
/// [`SoundComputer`] running the given program (vector of instructions).
fn solve_part1(instructions: &[Instruction]) -> usize {
    let mut sound_computer = SoundComputer::new(instructions, false);
    sound_computer.execute();
    sound_computer.get_mul_executions_count()
}

/// Solves AOC 2017 Day 23 Part 2.
///
/// Returns the value held in register "h" of SoundComputer after program execution halts, with
/// debug switch toggled off.
///
/// Optimised program counts the number of composite numbers (increasing by the step coded into the
/// second-last instruction) between a lower and upper limit. The lower and upper limits are
/// calculated based on the seed value coded into the first instruction of the program.
fn solve_part2(instructions: &[Instruction]) -> usize {
    // Extract seed and step values from the sound computer program
    let sound_comp = SoundComputer::new(instructions, false);
    let seed = sound_comp.extract_last_arg_value(0).unwrap().unsigned_abs();
    let step = sound_comp
        .extract_last_arg_value(instructions.len() - 2)
        .unwrap()
        .unsigned_abs();
    // Calculate lower and upper bounds for composite number check
    let lower = seed * 100 + 100000;
    let upper = lower + 17000;
    // Count composite numbers between upper and lower bound
    (lower..=upper)
        .step_by(step as usize)
        .map(is_composite)
        .filter(|&composite| composite)
        .count()
}

/// Checks if a given number n is prime.
fn is_composite(n: u64) -> bool {
    let upper = (n as f64).sqrt() as u64 + 1;
    for value in 2..=upper {
        if n % value == 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 23 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day23_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(6241, solution);
    }

    /// Tests the Day 23 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day23_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(909, solution);
    }
}
