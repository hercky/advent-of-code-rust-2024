advent_of_code::solution!(22);

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
    None
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
        assert_eq!(result, None);
    }
}
