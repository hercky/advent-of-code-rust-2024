advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    let mut sum = 0;

    for cap in re.captures_iter(input) {
        let a: u32 = cap["a"].parse().unwrap();
        let b: u32 = cap["b"].parse().unwrap();
        sum += a * b;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re_combined =
        Regex::new(r"(?<do>do\(\))|(?<dont>don't\(\))|mul\((?<a>\d+),(?<b>\d+)\)").unwrap();

    let mut sum = 0;
    let mut skip = false;

    for cap in re_combined.captures_iter(input) {
        if cap.name("do").is_some() {
            // do()
            skip = false;
        } else if cap.name("dont").is_some() {
            // don't()
            skip = true;
        } else {
            // mul(a,b)
            let a: u32 = cap["a"].parse().unwrap();
            let b: u32 = cap["b"].parse().unwrap();
            if !skip {
                sum += a * b;
            }
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
