use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

const PROBLEM_NAME: &str = "I Heard You Like Registers";
const PROBLEM_INPUT_FILE: &str = "./input/day08.txt";
const PROBLEM_DAY: u64 = 8;

lazy_static! {
    static ref REGEX_INSTRUCTION: Regex =
        Regex::new(r"^([a-z]+) (inc|dec) (-?\d+) if ([a-z]+) (>|>=|==|<|<=|!=) (-?\d+)$").unwrap();
}

/// Custom error type indicating that the parsing of an Instruction has failed.
#[derive(Debug)]
struct InstructionParseError;

/// Represents a single instruction used in this problem.
struct Instruction {
    reg_target: String,
    op: Operation,
    delta: i64,
    reg_check: String,
    comp: Comparator,
    check_value: i64,
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(Some(caps)) = REGEX_INSTRUCTION.captures(s) {
            let reg_target = caps[1].to_string();
            let op = Operation::from_str(&caps[2])?;
            let delta = caps[3].parse::<i64>().unwrap();
            let reg_check = caps[4].to_string();
            let comp = Comparator::from_str(&caps[5])?;
            let check_value = caps[6].parse::<i64>().unwrap();
            return Ok(Instruction {
                reg_target,
                op,
                delta,
                reg_check,
                comp,
                check_value,
            });
        }
        Err(InstructionParseError)
    }
}

/// Represents the operations that can be applied to a register value.
#[derive(Clone, Copy)]
enum Operation {
    Increase,
    Decrease,
}

impl FromStr for Operation {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(Operation::Increase),
            "dec" => Ok(Operation::Decrease),
            _ => Err(InstructionParseError),
        }
    }
}

/// Represents the comparators that occur in the instructions from the problem.
#[derive(Clone, Copy)]
enum Comparator {
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    LessThan,
    LessThanOrEqual,
    NotEqual,
}

impl FromStr for Comparator {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Comparator::GreaterThan),
            ">=" => Ok(Comparator::GreaterThanOrEqual),
            "==" => Ok(Comparator::Equal),
            "<" => Ok(Comparator::LessThan),
            "<=" => Ok(Comparator::LessThanOrEqual),
            "!=" => Ok(Comparator::NotEqual),
            _ => Err(InstructionParseError),
        }
    }
}

/// Processes the AOC 2017 Day 08 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 08 input file in the format required by the solver functions.
/// Returned value is vector of Instructions parsed from the lines of the input file.
fn process_input_file(filename: &str) -> Vec<Instruction> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| Instruction::from_str(line).unwrap())
        .collect::<Vec<Instruction>>()
}

/// Solves AOC 2017 Day 08 Part 1 // Determines the maximum register value after all instructions
/// have been processed.
fn solve_part1(instructions: &[Instruction]) -> i64 {
    if let (Some(max_at_end), _) = process_instructions(instructions) {
        return max_at_end;
    }
    panic!("Failed to find maximum register value at end of instruction processing!");
}

/// Solves AOC 2017 Day 08 Part 2 // Determines the maximum register value at any point during the
/// processing of the instructions.
fn solve_part2(instructions: &[Instruction]) -> i64 {
    if let (_, Some(max_during)) = process_instructions(instructions) {
        return max_during;
    }
    panic!("Failed to find maximum register value during processing of instructions!");
}

/// Processes the given instructions, returning the maximum value at the end of processing and
/// during processing as a tuple. Both tuple elements will be None if the input collection is empty.
fn process_instructions(instructions: &[Instruction]) -> (Option<i64>, Option<i64>) {
    let mut regs: HashMap<&str, i64> = HashMap::new();
    let mut max_value: Option<i64> = None;
    for instruct in instructions.iter() {
        // Add target register
        if let Entry::Vacant(e) = regs.entry(instruct.reg_target.as_str()) {
            e.insert(0);
        }
        // Add check register
        if let Entry::Vacant(e) = regs.entry(instruct.reg_check.as_str()) {
            e.insert(0);
        }
        let reg_check_value = *regs.get(instruct.reg_check.as_str()).unwrap();
        let update_reg = match instruct.comp {
            Comparator::GreaterThan => reg_check_value > instruct.check_value,
            Comparator::GreaterThanOrEqual => reg_check_value >= instruct.check_value,
            Comparator::Equal => reg_check_value == instruct.check_value,
            Comparator::LessThan => reg_check_value < instruct.check_value,
            Comparator::LessThanOrEqual => reg_check_value <= instruct.check_value,
            Comparator::NotEqual => reg_check_value != instruct.check_value,
        };
        if update_reg {
            *regs.get_mut(instruct.reg_target.as_str()).unwrap() += match instruct.op {
                Operation::Increase => instruct.delta,
                Operation::Decrease => -instruct.delta,
            };
        }
        // Check value of target register
        if max_value.is_none()
            || *regs.get(instruct.reg_target.as_str()).unwrap() > max_value.unwrap()
        {
            max_value = Some(*regs.get(instruct.reg_target.as_str()).unwrap());
        }
        // Check value of check register
        if max_value.is_none()
            || *regs.get(instruct.reg_check.as_str()).unwrap() > max_value.unwrap()
        {
            max_value = Some(*regs.get(instruct.reg_check.as_str()).unwrap());
        }
    }
    (regs.values().max().copied(), max_value)
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 08 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day08_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(4902, solution);
    }

    /// Tests the Day 08 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day08_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(7037, solution);
    }
}
