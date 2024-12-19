use std::collections::HashMap;

advent_of_code::solution!(11);

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<u64>().unwrap())
        .collect()
}

fn blink(stones: &Vec<u64>) -> Vec<u64> {
    let mut new_stones = Vec::new();

    for i in 0..stones.len() {
        if stones[i] == 0 {
            new_stones.push(1);
        } else if stones[i].to_string().len() % 2 == 0 {
            let num_str = stones[i].to_string();
            let (left, right) = num_str.split_at(num_str.len() / 2);
            new_stones.push(left.parse::<u64>().unwrap());
            new_stones.push(right.parse::<u64>().unwrap());
        } else {
            new_stones.push(stones[i] * 2024);
        }
    }

    new_stones
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut stones = parse_input(input);

    for i in 0..25 {
        stones = blink(&stones);
    }

    Some(stones.len() as u32)
}

fn fast_blink(stones: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::new();

    for (&stone, &count) in stones {
        let next_stone = match stone {
            0 => vec![1],
            _ if stone.to_string().len() % 2 == 0 => {
                let num_str = stone.to_string();
                let (left, right) = num_str.split_at(num_str.len() / 2);
                let left_num = left.parse::<u64>().unwrap();
                let right_num = right.parse::<u64>().unwrap();
                vec![left_num, right_num]
            }
            _ => vec![stone * 2024],
        };

        for stone in next_stone {
            *new_stones.entry(stone).or_insert(0) += count;
        }
    }

    new_stones
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut stones = HashMap::new();
    for stone in parse_input(input) {
        *stones.entry(stone).or_insert(0) += 1;
    }

    println!("{:?}", stones);

    for _ in 0..75 {
        stones = fast_blink(&stones);
    }

    let total = stones.values().sum::<u64>();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(239321955280205));
    }
}
