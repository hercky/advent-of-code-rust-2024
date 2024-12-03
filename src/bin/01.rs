use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    // println!("{}", input);

    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    for line in input.lines() {
        // remove the newline character
        let line = line.trim();
        let mut parts = line.split_whitespace();
        let l = parts.next().unwrap();
        let r = parts.next().unwrap();

        left.push(l.parse::<u32>().unwrap());
        right.push(r.parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    // println!("{:?}", left);
    // println!("{:?}", right);

    let mut sum = 0;
    for i in 0..left.len() {
        sum += left[i].abs_diff(right[i]);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: HashMap<u32, u32> = HashMap::new();
    let mut right: HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        let line = line.trim();
        let mut parts = line.split_whitespace();
        let l = parts.next().unwrap().parse::<u32>().unwrap();
        let r = parts.next().unwrap().parse::<u32>().unwrap();
        // println!("{} {}", l, r);

        if left.contains_key(&l) {
            left.insert(l, left.get(&l).unwrap() + 1);
        } else {
            left.insert(l, 1);
        }
        if right.contains_key(&r) {
            right.insert(r, right.get(&r).unwrap() + 1);
        } else {
            right.insert(r, 1);
        }
    }

    // println!("{:?}", left);
    // println!("{:?}", right);

    let mut sum = 0;
    for (k, v) in left.iter() {
        if right.contains_key(k) {
            sum += (k * v * right.get(k).unwrap());
        } else {
            sum += 0;
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
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
