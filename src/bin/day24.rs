use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::{fs, vec};
use std::time::Instant;

use fancy_regex::Regex;
use lazy_static::lazy_static;

use aoc2017::utils::error::InputFileParseError;

const PROBLEM_NAME: &str = "Electromagnetic Moat";
const PROBLEM_INPUT_FILE: &str = "./input/day24.txt";
const PROBLEM_DAY: u64 = 24;

lazy_static! {
    static ref REGEX_COMP: Regex = Regex::new(r"^(\d+)/(\d+)$").unwrap();
}

/// Represents a single component with two ports.
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Component {
    ports: [u64; 2],
}

impl Component {
    /// Creates a new [`Component`] with ports sorted in ascending order.
    fn new(ports: &[u64; 2]) -> Self {
        let mut ports = *ports;
        ports.sort();
        Self { ports }
    }

    /// Gets the strength of the [`Component`] by adding together the number of pins on its ports.
    fn get_strength(&self) -> u64 {
        self.ports.iter().sum()
    }

    /// Gets the lower port pin number.
    fn get_lower(&self) -> u64 {
        self.ports[0]
    }

    /// Gets the upper port pin number.
    fn get_upper(&self) -> u64 {
        self.ports[1]
    }

    /// Gets the other port pin count for the given pin value.
    ///
    /// Returns None if the given pin value is not present on the [`Component`].
    fn get_other_pins(&self, pins: u64) -> Option<u64> {
        if self.ports[0] == self.ports[1] && self.ports[0] == pins {
            Some(self.ports[0])
        } else if self.ports[0] == pins && self.ports[1] != pins {
            Some(self.ports[1])
        } else if self.ports[0] != pins && self.ports[1] == pins {
            Some(self.ports[0])
        } else {
            None
        }
    }
}

/// Processes the AOC 2017 Day 24 input file and solves both parts of the problem. Solutions are
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

/// Processes the AOC 2017 Day 24 input file in the format required by the solver functions.
///
/// Returned value is HashSet mapping each observed port pin number to the components with that many
/// pins. Since each [`Component`] has two ports, a copy of the [`Component`] is added to the values
/// corresponding to its port pin numbers, where those numbers are different.
///
/// Assumption made that all components are unique, i.e., no two components have ports with the same
/// combination of pin numbers.
fn process_input_file(filename: &str) -> HashMap<u64, Vec<Component>> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    let mut components: HashMap<u64, Vec<Component>> = HashMap::new();
    for line in raw_input
        .lines()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        // Create component and get the lower and upper port pin values
        let comp = parse_input_line(line).unwrap();
        let (lower, upper) = (comp.get_lower(), comp.get_upper());
        // Add the component to the output map
        add_component_to_output_map(&mut components, lower, comp);
        if lower != upper {
            add_component_to_output_map(&mut components, upper, comp);
        }
        components
            .get_mut(&lower)
            .unwrap_or(&mut Vec::<Component>::new())
            .push(comp.clone());
        if lower != upper {
            components
                .get_mut(&upper)
                .unwrap_or(&mut Vec::<Component>::new())
                .push(comp.clone());
        }
    }
    components
}

/// Adds the component to the output map.
fn add_component_to_output_map(components: &mut HashMap<u64, Vec<Component>>, pin_count: u64, comp: Component) {
    if let Entry::Vacant(e) = components.entry(pin_count) {
        e.insert(vec![comp]);
    } else {
        components.get_mut(&pin_count).unwrap().push(comp);
    }
}

/// Parses a single input line to extract the [`Component`] encoded by the line.
///
/// If the string is incorrectly formatted, an [`InputFileParseError`] is returned.
fn parse_input_line(s: &str) -> Result<Component, InputFileParseError> {
    if let Ok(Some(caps)) = REGEX_COMP.captures(s) {
        let left = caps[1].parse::<u64>().unwrap();
        let right = caps[2].parse::<u64>().unwrap();
        let ports: [u64; 2] = [left, right];
        let component = Component::new(&ports);
        return Ok(component);
    }
    Err(InputFileParseError {
        message: format!("Invalid format for input file line: {s}"),
    })
}

/// Solves AOC 2017 Day 24 Part 1.
///
/// Finds the strength of the strongest bridge that can be made.
fn solve_part1(components: &HashMap<u64, Vec<Component>>) -> u64 {
    find_strongest_bridge(components)
}

fn find_strongest_bridge(components: &HashMap<u64, Vec<Component>>) -> u64 {
    let mut max_strength: Option<u64> = None;
    let zero_comp = components.get(&0).unwrap()[0];
    let strength = zero_comp.get_strength();
    let next_pins = zero_comp.get_upper();
    let observed: HashSet<Component> = HashSet::from([zero_comp.clone()]);
    find_strongest_bridge_recursive(
        components,
        &observed,
        strength,
        &mut max_strength,
        next_pins,
    );
    max_strength.unwrap()
}

fn find_strongest_bridge_recursive(
    components: &HashMap<u64, Vec<Component>>,
    observed: &HashSet<Component>,
    strength: u64,
    max_strength: &mut Option<u64>,
    next_pins: u64,
) {
    // Check the candidates
    for next_comp in components.get(&next_pins).unwrap() {
        if observed.contains(next_comp) {
            continue;
        }
        // Record next component as observed
        let mut observed = observed.clone();
        observed.insert(next_comp.clone());
        // Get the next exposed pins value
        let other_pins = next_comp.get_other_pins(next_pins).unwrap();
        let strength = strength + next_comp.get_strength();
        find_strongest_bridge_recursive(components, &observed, strength, max_strength, other_pins);
    }
    // Bridge cannot be extended further, so check if new maximum strength has been found
    if max_strength.is_none() || max_strength.unwrap() < strength {
        println!("New maximum strength found: {strength}");
        *max_strength = Some(strength);
    }
}

/// Solves AOC 2017 Day 24 Part 2.
///
/// ###
fn solve_part2(_input: &HashMap<u64, Vec<Component>>) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 24 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day24_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(1695, solution);
    }

    /// Tests the Day 24 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day24_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(1673, solution);
    }
}
