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

pub fn part_one(input: &str) -> Option<u64> {
    let lines = parse_input(input);

    let (numeric_codes, directional_codes) = find_codes();

    let mut complexity: u64 = 0;

    for line in lines {
        let code = numeric_code_to_directional_code(&line, &numeric_codes);
        let code = directional_to_directional_code(&code, &directional_codes);
        let code = directional_to_directional_code(&code, &directional_codes);

        complexity += line[..line.len() - 1].parse::<u64>().unwrap() * code.len() as u64;
    }

    Some(complexity)
}

pub fn part_two(input: &str) -> Option<u64> {
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
