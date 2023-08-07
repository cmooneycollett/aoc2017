use std::fs;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "Dueling Generators";
const PROBLEM_INPUT_FILE: &str = "./input/day15.txt";
const PROBLEM_DAY: u64 = 15;

/// Number of generator rounds conducted in problem part 1
const PART1_ROUNDS: u64 = 40_000_000;
/// Number of generator rounds conducted in problem part 2
const PART2_ROUNDS: u64 = 5_000_000;
/// Factor used by the A generator
const GEN_A_FACTOR: u64 = 16_807;
/// Factor used by the B generator
const GEN_B_FACTOR: u64 = 48_271;
/// Modulus value used by both generators
const GEN_MODULUS: u64 = 2_147_483_647;

lazy_static! {
    static ref INPUT_REGEX: Regex =
        Regex::new(r"(?ms)^Generator A starts with (\d+)$.*^Generator B starts with (\d+)$")
            .unwrap();
}

/// Custom error type indicating that the parsing of the input file has failed.
#[derive(Debug)]
struct InputFileParseError;

/// Value generator used in the AOC 2017 Day 15 problem.
struct ValueGenerator {
    value: u64,
    factor: u64,
    modulus: u64,
    check_fn: fn(u64) -> bool,
}

impl ValueGenerator {
    /// Creates a new ValueGenerator.
    pub fn new(value: u64, factor: u64, modulus: u64, check_fn: fn(u64) -> bool) -> ValueGenerator {
        ValueGenerator {
            value,
            factor,
            modulus,
            check_fn,
        }
    }
}

impl Iterator for ValueGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.value = (self.value * self.factor) % self.modulus;
            if (self.check_fn)(self.value) {
                return Some(self.value);
            }
        }
    }
}

/// Processes the AOC 2017 Day 15 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 15 input file in the format required by the solver functions.
///
/// Returned value is a tuple containing the start values for the A and B generators.
fn process_input_file(filename: &str) -> (u64, u64) {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    parse_input_string(&raw_input).unwrap()
}

/// Solves AOC 2017 Day 15 Part 1.
///
/// Determines the number of matching values from the A and B generators after 40 million pairs of
/// values have been generated.
fn solve_part1(input: &(u64, u64)) -> usize {
    let (gen_a_start, gen_b_start) = *input;
    let mut gen_a = ValueGenerator::new(gen_a_start, GEN_A_FACTOR, GEN_MODULUS, |_| true);
    let mut gen_b = ValueGenerator::new(gen_b_start, GEN_B_FACTOR, GEN_MODULUS, |_| true);
    count_matching_value_pairs(&mut gen_a, &mut gen_b, PART1_ROUNDS)
}

/// Solves AOC 2017 Day 15 Part 2.
///
/// Determines the number of matching values from the A and B generators after 5 million pairs, with
/// each generator using a non-trivial value-checking function.
fn solve_part2(input: &(u64, u64)) -> usize {
    let (gen_a_start, gen_b_start) = *input;
    let mut gen_a = ValueGenerator::new(gen_a_start, GEN_A_FACTOR, GEN_MODULUS, |v| v % 4 == 0);
    let mut gen_b = ValueGenerator::new(gen_b_start, GEN_B_FACTOR, GEN_MODULUS, |v| v % 8 == 0);
    count_matching_value_pairs(&mut gen_a, &mut gen_b, PART2_ROUNDS)
}

/// Parses the contents of the input file and returns the values needed by the solution functions.
///
/// If the input file string is correctly formatted, a tuple containing the start values for the A
/// and B generators is returned. Otherwise, an [`InputFileParseError`] is returned.
fn parse_input_string(s: &str) -> Result<(u64, u64), InputFileParseError> {
    if let Ok(Some(caps)) = INPUT_REGEX.captures(s) {
        let val_a = caps[1].parse::<u64>().unwrap();
        let val_b = caps[2].parse::<u64>().unwrap();
        return Ok((val_a, val_b));
    }
    Err(InputFileParseError)
}

/// Counts the number of matching value pairs return by the two generators after the specified
/// number of rounds have been conducted. Only the lowest 16 bits of the values returned by the
/// generators need to be the same for a match to be recorded.
fn count_matching_value_pairs(
    gen_a: &mut ValueGenerator,
    gen_b: &mut ValueGenerator,
    total_rounds: u64,
) -> usize {
    let mut matches = 0;
    for _ in 0..total_rounds {
        // Get the next value from both generators
        let gen_a_value = gen_a.next().unwrap();
        let gen_b_value = gen_b.next().unwrap();
        // Compare the lowest 16 bits of the generator values
        if gen_a_value & 0xffff == gen_b_value & 0xffff {
            matches += 1;
        }
    }
    matches
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 15 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day15_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(594, solution);
    }

    /// Tests the Day 15 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day15_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(328, solution);
    }
}
