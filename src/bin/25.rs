advent_of_code::solution!(25);

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum LockOrKey {
    Lock([u8; 5]),
    Key([u8; 5]),
}

fn is_a_match(lock: &LockOrKey, key: &LockOrKey) -> bool {
    assert!(matches!(lock, LockOrKey::Lock(_)));
    assert!(matches!(key, LockOrKey::Key(_)));

    if let (LockOrKey::Lock(lock_arr), LockOrKey::Key(key_arr)) = (lock, key) {
        for i in 0..5 {
            if lock_arr[i] + key_arr[i] > 7 {
                return false;
            }
        }
    }

    true
}

fn parse_input(input: &str) -> Vec<LockOrKey> {
    let mut lines = input.lines();
    let mut result = Vec::new();

    loop {
        let mut schema = Vec::new();
        for _ in 0..7 {
            let line = lines.next().unwrap();
            schema.push(line);
        }

        let is_lock = schema[0].starts_with("#");
        let mut numeric_schema: [u8; 5] = [0; 5];

        for col in 0..5 {
            numeric_schema[col] = schema
                .iter()
                .filter(|row| {
                    if is_lock {
                        row.chars().nth(col).unwrap() == '#'
                    } else {
                        // row.chars().nth(col).unwrap() == '.'
                        row.chars().nth(col).unwrap() == '#'
                    }
                })
                .count() as u8;
        }

        if is_lock {
            result.push(LockOrKey::Lock(numeric_schema));
        } else {
            result.push(LockOrKey::Key(numeric_schema));
        }

        if lines.next().is_none() {
            break;
        }
    }

    result
}

pub fn part_one(input: &str) -> Option<u32> {
    let lock_and_keys = parse_input(input);

    let locks = lock_and_keys
        .iter()
        .filter(|lock| matches!(lock, LockOrKey::Lock(_)))
        .collect_vec();
    let keys = lock_and_keys
        .iter()
        .filter(|key| matches!(key, LockOrKey::Key(_)))
        .collect_vec();

    // println!("locks");
    // for lock in locks.iter() {
    //     println!("{:?}", lock);
    // }

    // println!("keys");
    // for key in keys.iter() {
    //     println!("{:?}", key);
    // }

    let mut result = 0;

    for lock in locks.iter() {
        for key in keys.iter() {
            if is_a_match(lock, key) {
                result += 1;
            }
        }
    }

    Some(result)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
