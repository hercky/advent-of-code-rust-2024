advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut ans = 0;

    for line in input.lines() {
        let line = line.trim();
        let (left, right) = line.split_once(':').unwrap();
        let left = left.parse::<u64>().unwrap();
        let right = right
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let mut dp: Vec<u64> = Vec::new();

        dp.push(right[0]);

        for i in 1..right.len() {
            let candidate = right[i];
            let mut new_dp: Vec<u64> = Vec::new();

            for j in 0..dp.len() {
                let prev = dp[j];
                // *
                new_dp.push(prev * candidate);
                // +
                new_dp.push(prev + candidate);
            }

            dp = new_dp;
        }

        for j in 0..dp.len() {
            if dp[j] == left {
                ans += left;
                break;
            }
        }
    }

    Some(ans)
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
