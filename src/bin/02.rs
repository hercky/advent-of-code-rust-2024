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

fn adjacent_safe(left: i32, right: i32, desc: bool) -> bool {
    if (desc && right < left) || (!desc && right > left) {
        return false;
    }

    let diff = right.abs_diff(left);
    if diff < 1 || diff > 3 {
        return false;
    }

    true
}

pub fn check_line_safe(a: &[i32], desc: Option<bool>) -> bool {
    let asc = desc.unwrap_or_else(|| a[1] > a[0]);

    for i in 1..a.len() {
        if !adjacent_safe(a[i - 1], a[i], asc) {
            return false;
        }
    }

    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut answer = 0;
    // if there is problem in the beginning, strike - 1, and send the rest of the line to check_line_safe // asc none

    // if problem in the middle, strike -1, and slice and join the rest of the line and send to check_line_safe

    for line in input.lines() {
        let mut line_pass = true;

        let line = line.trim();
        let v: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let desc = v[1] > v[0];

        // convert v to array
        let a = &v[..];

        for i in 1..a.len() {
            let left = a[i - 1];
            let right = a[i];

            if adjacent_safe(left, right, desc) {
                continue;
            } else {
                // something unsafe
                let mut pass = false;
                for j in 0..a.len() {
                    if check_line_safe(&[&a[0..j], &a[j + 1..]].concat(), None) {
                        pass = true;
                        break;
                    }
                }
                line_pass = pass;
                break;
            }
        }

        if line_pass {
            answer += 1;
        }
    }

    Some(answer as u32)
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
        assert_eq!(result, Some(4));
    }
}
