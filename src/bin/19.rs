advent_of_code::solution!(19);

use std::collections::HashMap;

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let (patterns, combinations) = input.split_once("\n\n").unwrap();
    (
        patterns.split(",").map(|s| s.trim().to_string()).collect(),
        combinations.lines().map(|line| line.to_string()).collect(),
    )
}

fn split_string(input_string: &str, index: usize) -> (String, String) {
    let (prefix, suffix) = input_string.split_at(index);
    (prefix.to_string(), suffix.to_string())
}

fn check_combination_possible(patterns: &mut HashMap<String, bool>, combination: &str) -> bool {
    let max_len = combination.len();

    if patterns.contains_key(combination) {
        return patterns[combination];
    }

    for len in 1..max_len {
        let (prefix, suffix) = split_string(combination, len);

        if check_combination_possible(patterns, &prefix)
            && check_combination_possible(patterns, &suffix)
        {
            patterns.insert(combination.to_string(), true);
            return true;
        }
    }
    patterns.insert(combination.to_string(), false);

    patterns[combination]
}

pub fn part_one(input: &str) -> Option<u32> {
    let (patterns, combinations) = parse_input(input);

    let mut answer = 0;

    let mut patterns_map = HashMap::new();
    patterns_map.insert("".to_string(), true);
    for pattern in patterns {
        patterns_map.insert(pattern, true);
    }

    // println!("{:?}", patterns_map);

    for combination in combinations {
        if check_combination_possible(&mut patterns_map, &combination) {
            answer += 1;
        }
        println!("{}", patterns_map.len());
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
