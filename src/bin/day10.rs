use std::fs;
use std::time::Instant;

use itertools::Itertools;

const PROBLEM_NAME: &str = "Knot Hash";
const PROBLEM_INPUT_FILE: &str = "./input/day10.txt";
const PROBLEM_DAY: u64 = 10;

/// Processes the AOC 2017 Day 10 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 10 input file in the format required by the solver functions.
/// Returned value is string contained in the input file.
fn process_input_file(filename: &str) -> String {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input.trim().to_string()
}

/// Solves AOC 2017 Day 10 Part 1 // Calculates the sparse hash of the numbers 0-255 inclusive using
/// the comma-separated values in the input string, and returns the product of the first two values
/// of the sparse hash.
fn solve_part1(input_string: &str) -> u64 {
    let lengths = input_string
        .split(',')
        .map(|val| val.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let strand = (0..=255).collect::<Vec<u64>>();
    let (strand, _, _) = calculate_sparse_hash(&strand, &lengths, 0, 0);
    strand[0] * strand[1]
}

/// Solves AOC 2017 Day 10 Part 2 // Calculates the knot hash of the given string.
fn solve_part2(input_string: &str) -> String {
    calculate_knot_hash(input_string)
}

/// Calculates the sparse hash of the given strand. Returns the resulting strand, final cursor
/// value, and final skip value.
fn calculate_sparse_hash(
    strand: &[u64],
    lengths: &[usize],
    cursor: usize,
    skip: usize,
) -> (Vec<u64>, usize, usize) {
    let mut strand = strand.to_vec();
    let mut cursor = cursor;
    let mut skip = skip;
    for &len in lengths {
        // Reverse target segment
        let mut segment: Vec<u64> = vec![];
        for delta in 0..len {
            let i = (cursor + delta) % strand.len();
            segment.push(strand[i]);
        }
        segment.reverse();
        for (j, &segment_elem) in segment.iter().enumerate() {
            let i = (cursor + j) % strand.len();
            strand[i] = segment_elem;
        }
        // Update cursor location and increment skip value
        cursor = (cursor + len + skip) % strand.len();
        skip += 1;
    }
    (strand, cursor, skip)
}

/// Calculates the knot hash of the input string, including input processing (length sequence suffix
/// append), 64 rounds of knot hash algorithm and output processing (dense hash calculation and
/// conversion to hexadecimal string).
fn calculate_knot_hash(input_string: &str) -> String {
    // Input processing
    let mut lengths = input_string
        .chars()
        .map(|c| c as usize)
        .collect::<Vec<usize>>();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);
    // Apply 64 rounds of the sparse hash algorithm
    let mut strand = (0..=255).collect::<Vec<u64>>();
    let mut cursor = 0;
    let mut skip = 0;
    for _ in 0..64 {
        (strand, cursor, skip) = calculate_sparse_hash(&strand, &lengths, cursor, skip);
    }
    // Convert to dense hash
    let mut dense_hash: Vec<u64> = vec![];
    for i in (0..strand.len()).step_by(16) {
        let mut xor = strand[i];
        for delta in 1..=15 {
            xor ^= strand[i + delta];
        }
        dense_hash.push(xor);
    }
    // Convert dense hash to hexadecimal representation
    dense_hash.iter().map(|val| format!("{:02x}", val)).join("")
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 10 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day10_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(38628, solution);
    }

    /// Tests the Day 10 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day10_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!("e1462100a34221a7f0906da15c1c979a", solution);
    }
}
