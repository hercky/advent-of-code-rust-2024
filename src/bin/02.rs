advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut answer: i32 = 0;

    for line in input.lines() {
        let mut line_pass = true;

        let line = line.trim();
        let v: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let asc = v[1] > v[0];

        for i in 1..v.len() {
            // check for the sorted order
            if (asc && v[i] < v[i - 1]) || (!asc && v[i] > v[i - 1]) {
                line_pass = false;
                break;
            }
            // check for the problem condition
            let diff = v[i].abs_diff(v[i - 1]);
            if diff < 1 || diff > 3 {
                line_pass = false;
                break;
            }
        }

        if line_pass {
            answer += 1;
        }
    }

    Some(answer as u32)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
