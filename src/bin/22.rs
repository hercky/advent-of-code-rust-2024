advent_of_code::solution!(22);

use std::collections::{HashMap, HashSet};

// Hint from reddit: The problem is based on the linear feedback shift register
// https://en.wikipedia.org/wiki/Linear_feedback_shift_register
/// [Xorshift LFSR](https://en.wikipedia.org/wiki/Linear-feedback_shift_register#Xorshift_LFSRs).
fn hash(mut n: usize) -> usize {
    n = (n ^ (n << 6)) & 0xffffff;
    n = (n ^ (n >> 5)) & 0xffffff;
    (n ^ (n << 11)) & 0xffffff
}

fn get_2k_number(n: usize) -> usize {
    let mut number = n;
    for _ in 0..2000 {
        number = hash(number);
    }
    number
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut numbers = Vec::new();

    for line in input.lines() {
        numbers.push(line.parse::<usize>().unwrap());
    }

    let mut answer = 0;

    for number in numbers {
        let n = get_2k_number(number);
        answer += n;
        // println!("{}", n);
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u64> {
    let numbers: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut total_bananas: HashMap<Vec<i32>, u64> = HashMap::new();

    for &num in &numbers {
        let mut seen = HashSet::new();
        let mut diffs = Vec::new();
        let mut current = num;

        for i in 0..2000 {
            let next = hash(current);
            diffs.push(((next % 10) as i32) - ((current % 10) as i32));
            current = next;

            if i >= 3 {
                let sequence = diffs.clone();
                if !seen.contains(&sequence) {
                    seen.insert(sequence.clone());
                    *total_bananas.entry(sequence).or_default() += (current % 10) as u64;
                }
                diffs.remove(0);
            }
        }
    }

    total_bananas.values().max().copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
