advent_of_code::solution!(9);

fn find_next_negative_index(repr: &Vec<i32>, prev_index: i32) -> Option<usize> {
    for i in (prev_index + 1) as usize..repr.len() {
        if repr[i] < 0 {
            return Some(i);
        }
    }
    None
}

fn find_prev_negative_index(repr: &Vec<i32>, prev_index: i32) -> Option<usize> {
    for i in (0..(prev_index) as usize).rev() {
        if repr[i] >= 0 {
            return Some(i);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let orig_code = input.trim().to_string();

    let mut orig_repr: Vec<i32> = Vec::new();

    let mut free: bool = false;
    let mut id: i32 = 0;

    for c in orig_code.chars() {
        let repeat_count = c.to_digit(10).unwrap() as i32;

        if !free {
            for _ in 0..repeat_count {
                orig_repr.push(id);
            }

            free = true;
            id += 1;
        } else {
            for _ in 0..repeat_count {
                orig_repr.push(-1);
            }
            free = false;
        }
    }

    // println!("{:?}", orig_repr);

    let mut front_index = find_next_negative_index(&orig_repr, -1)
        .or(Some(orig_repr.len() - 1))
        .unwrap();
    let mut end_index = find_prev_negative_index(&orig_repr, orig_repr.len() as i32)
        .or(Some(0))
        .unwrap();

    while front_index < end_index {
        // swap front and end
        let temp = orig_repr[front_index];
        orig_repr[front_index] = orig_repr[end_index];
        orig_repr[end_index] = temp;

        // find next front and end
        let next_front = find_next_negative_index(&orig_repr, front_index as i32);
        let next_end = find_prev_negative_index(&orig_repr, end_index as i32);

        // Break if either index can't be found
        if next_front.is_none() || next_end.is_none() {
            break;
        }

        front_index = next_front.unwrap();
        end_index = next_end.unwrap();
    }

    // println!("{:?}", orig_repr);

    let mut ans: u64 = 0;
    for i in 0..orig_repr.len() {
        if orig_repr[i] >= 0 {
            ans += orig_repr[i] as u64 * i as u64;
        }
    }
    Some(ans)
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
