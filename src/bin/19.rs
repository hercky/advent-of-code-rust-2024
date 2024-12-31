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
    // patterns_map.insert("".to_string(), true);
    for pattern in patterns {
        patterns_map.insert(pattern, true);
    }

    // println!("{:?}", patterns_map);

    for combination in combinations {
        if check_combination_possible(&mut patterns_map, &combination) {
            answer += 1;
        }
    }

    Some(answer)
}

fn count_combinations(
    original_patterns: &Vec<String>,
    all_patterns_map: &mut HashMap<String, u64>,
    combination: &str,
) -> u64 {
    if combination.len() == 0 {
        return 1;
    }

    if all_patterns_map.contains_key(combination) {
        return all_patterns_map[combination];
    }

    let mut count = 0;

    for pattern in original_patterns {
        if combination.starts_with(pattern) {
            let suffix = combination[pattern.len()..].to_string();
            count += count_combinations(original_patterns, all_patterns_map, &suffix);
        }
    }

    all_patterns_map.insert(combination.to_string(), count);

    count
}

pub fn part_two(input: &str) -> Option<u64> {
    let (patterns, combinations) = parse_input(input);

    let mut answer = 0;
    let mut patterns_map: HashMap<String, u64> = HashMap::new();

    let patterns_vec = patterns.clone();

    for combination in combinations {
        let combination_count = count_combinations(&patterns_vec, &mut patterns_map, &combination);
        println!("{} -> {}", combination, combination_count);
        answer += combination_count;

        // break;
    }

    // println!("patterns_map");
    // for (pattern, count) in &patterns_map {
    //     println!("{} -> {}", pattern, count);
    // }

    Some(answer)
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
        assert_eq!(result, Some(16));
    }
}
