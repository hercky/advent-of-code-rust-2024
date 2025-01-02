advent_of_code::solution!(21);

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref NUMERIC: HashMap<char, (usize, usize)> = HashMap::from([
        ('7', (0, 0)),
        ('8', (0, 1)),
        ('9', (0, 2)),
        ('4', (1, 0)),
        ('5', (1, 1)),
        ('6', (1, 2)),
        ('1', (2, 0)),
        ('2', (2, 1)),
        ('3', (2, 2)),
        ('0', (3, 1)),
        ('A', (3, 2)),
    ]);
    static ref DIRECTIONAL: HashMap<char, (usize, usize)> = HashMap::from([
        ('^', (0, 1)),
        ('A', (0, 2)),
        ('<', (1, 0)),
        ('v', (1, 1)),
        ('>', (1, 2)),
    ]);
}

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|line| line.trim().to_string()).collect()
}

fn find_codes() -> (HashMap<(char, char), String>, HashMap<(char, char), String>) {
    let mut numeric_codes = HashMap::new();
    let mut directional_codes = HashMap::new();

    for (prev, (x, y)) in NUMERIC.iter() {
        for (next, (nx, ny)) in NUMERIC.iter() {
            let mut path = "<".repeat(y.saturating_sub(*ny))
                + &"v".repeat(nx.saturating_sub(*x))
                + &"^".repeat(x.saturating_sub(*nx))
                + &">".repeat(ny.saturating_sub(*y));
            if (*x, *ny) == (3, 0) || (*nx, *y) == (3, 0) {
                path = path.chars().rev().collect();
            }
            numeric_codes.insert((*prev, *next), path + "A");
        }
    }

    for (prev, (x, y)) in DIRECTIONAL.iter() {
        for (next, (nx, ny)) in DIRECTIONAL.iter() {
            let mut path = "<".repeat(y.saturating_sub(*ny))
                + &"v".repeat(nx.saturating_sub(*x))
                + &"^".repeat(x.saturating_sub(*nx))
                + &">".repeat(ny.saturating_sub(*y));
            if (*nx, *y) == (0, 0) || (*x, *ny) == (0, 0) {
                path = path.chars().rev().collect();
            }
            directional_codes.insert((*prev, *next), path + "A");
        }
    }

    (numeric_codes, directional_codes)
}

fn numeric_code_to_directional_code(
    code: &str,
    numeric_codes: &HashMap<(char, char), String>,
) -> String {
    let mut result = String::new();
    let mut prev = 'A';
    for c in code.chars() {
        result.push_str(&numeric_codes[&(prev, c)]);
        prev = c;
    }
    result
}

fn directional_to_directional_code(
    code: &str,
    directional_codes: &HashMap<(char, char), String>,
) -> String {
    let mut result = String::new();
    let mut prev = 'A';
    for c in code.chars() {
        result.push_str(&directional_codes[&(prev, c)]);
        prev = c;
    }
    result
}

// Add this new struct to store memoization results
#[derive(Default)]
struct Memo {
    cache: HashMap<(String, u32, bool), u128>,
}

impl Memo {
    fn get_length(
        &mut self,
        sequence: &str,
        iterations: u32,
        first_iter: bool,
        numeric_codes: &HashMap<(char, char), String>,
        directional_codes: &HashMap<(char, char), String>,
    ) -> u128 {
        // Check cache first
        let key = (sequence.to_string(), iterations, first_iter);
        if let Some(&result) = self.cache.get(&key) {
            return result;
        }

        // Base case
        if iterations == 0 {
            return sequence.len() as u128;
        }

        // Recursive case
        let mut total_length = 0;
        let mut prev = 'A';
        let codes = if first_iter {
            numeric_codes
        } else {
            directional_codes
        };

        for c in sequence.chars() {
            let next_sequence = &codes[&(prev, c)];
            total_length += self.get_length(
                next_sequence,
                iterations - 1,
                false,
                numeric_codes,
                directional_codes,
            );
            prev = c;
        }

        // Store in cache and return
        self.cache.insert(key, total_length);
        total_length
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    let lines = parse_input(input);
    let (numeric_codes, directional_codes) = find_codes();
    let mut memo = Memo::default();

    let mut complexity: u128 = 0;
    for line in lines {
        let number = line[..line.len() - 1].parse::<u128>().unwrap();
        complexity += number * memo.get_length(&line, 3, true, &numeric_codes, &directional_codes);
    }

    Some(complexity)
}

pub fn part_two(input: &str) -> Option<u128> {
    let lines = parse_input(input);
    let (numeric_codes, directional_codes) = find_codes();
    let mut memo = Memo::default();

    let mut complexity: u128 = 0;
    for line in lines {
        let number = line[..line.len() - 1].parse::<u128>().unwrap();
        complexity += number * memo.get_length(&line, 26, true, &numeric_codes, &directional_codes);
    }

    println!("--------------------------------");
    println!("{}", complexity);
    println!("--------------------------------");

    // Some(complexity)
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
