use std::process::id;

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

fn checksum(repr: &Vec<i32>) -> u64 {
    let mut ans: u64 = 0;
    for i in 0..repr.len() {
        if repr[i] >= 0 {
            ans += repr[i] as u64 * i as u64;
        }
    }
    ans
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
    Some(checksum(&orig_repr))
}

////// Part 2 //////

enum Block {
    Space(usize, usize),
    File(usize, usize, i32),
}

#[derive(Debug, Clone, Copy)]
struct File {
    start: usize,
    end: usize,
    id: i32,
}

#[derive(Debug, Clone, Copy)]
struct Space {
    start: usize,
    end: usize,
}

trait ChunkSize {
    fn size(&self) -> usize;
}

impl ChunkSize for File {
    fn size(&self) -> usize {
        self.end - self.start
    }
}

impl ChunkSize for Space {
    fn size(&self) -> usize {
        self.end - self.start
    }
}

impl ChunkSize for Block {
    fn size(&self) -> usize {
        match self {
            Block::Space(start, end) | Block::File(start, end, _) => end - start,
        }
    }
}

impl Block {
    fn debug_print(&self) -> String {
        match self {
            Block::Space(start, end) => format!("(-1 {})", end - start),
            Block::File(start, end, id) => format!("({} {})", id, end - start),
        }
    }
}

impl Space {
    fn debug_print(&self) -> String {
        format!(
            "(-1 {} at ({}, {}))",
            self.end - self.start,
            self.start,
            self.end
        )
    }
}

impl File {
    fn debug_print(&self) -> String {
        format!(
            "({} {} at ({}, {}))",
            self.id,
            self.end - self.start,
            self.start,
            self.end
        )
    }
}

fn convert_block_to_repr(blocks: &Vec<Block>) -> Vec<i32> {
    let mut repr: Vec<i32> = Vec::new();

    for block in blocks {
        match block {
            Block::Space(start, end) => repr.extend_from_slice(&vec![-1; end - start]),
            Block::File(start, end, id) => repr.extend_from_slice(&vec![*id; end - start]),
        }
    }

    repr
}

pub fn part_two(input: &str) -> Option<u64> {
    let orig_code = input.trim().to_string();

    let mut orig_repr: Vec<i32> = Vec::new();

    let mut free: bool = false;
    let mut id: i32 = 0;

    let mut files: Vec<File> = Vec::new();
    let mut spaces: Vec<Space> = Vec::new();

    for c in orig_code.chars() {
        let repeat_count = c.to_digit(10).unwrap() as i32;
        let start = orig_repr.len();
        let end = start + repeat_count as usize;

        if !free {
            // create string rep for the input
            orig_repr.extend(vec![id; repeat_count as usize]);
            files.push(File { start, end, id });
            free = true;
            id += 1;
        } else {
            orig_repr.extend(vec![-1; repeat_count as usize]);
            spaces.push(Space { start, end });
            free = false;
        }
    }

    // println!(
    //     "Blocks: {}",
    //     files
    //         .iter()
    //         .map(|b| b.debug_print())
    //         .collect::<Vec<_>>()
    //         .join(" ")
    // );
    // println!(
    //     "Spaces: {}",
    //     spaces
    //         .iter()
    //         .map(|s| s.debug_print())
    //         .collect::<Vec<_>>()
    //         .join(" ")
    // );

    // println!("{:?}", orig_repr);
    // done with parsing

    for candidate_file in files.iter_mut().rev() {
        let mut target_found = false;

        // println!("Candidate file {:?}", candidate_file);

        for target_space_index in 0..spaces.len() {
            let target_space = spaces[target_space_index];
            if target_space.size() >= candidate_file.size()
                && target_space.start < candidate_file.start
            {
                // println!("--------------------------------");
                // println!("Found target space {:?}", target_space);
                // println!(
                //     "Moving file of size {} with id {} at index ({}, {}) -> space of size {} at index ({}, {})",
                //     candidate_file.size(),
                //     candidate_file.id,
                //     candidate_file.start,
                //     candidate_file.end,
                //     target_space.size(),
                //     target_space.start,
                //     target_space.end
                // );

                // offset
                let target_space_size = target_space.size();
                let candidate_file_size = candidate_file.size();
                let leftover_capacity = target_space_size - candidate_file_size;

                // swap the files
                candidate_file.start = target_space.start;
                candidate_file.end = target_space.start + candidate_file_size;

                if leftover_capacity > 0 {
                    let new_space = Space {
                        start: target_space.start + candidate_file_size,
                        end: target_space.end,
                    };

                    // replace the target space with the new space
                    spaces[target_space_index] = new_space;
                } else {
                    assert!(leftover_capacity == 0);
                    // remove the target space
                    spaces.remove(target_space_index);
                }

                break;
            }
        }
        // println!("--------------------------------");
        // println!("{:?}", spaces);
        // println!("--------------------------------");
    }

    // println!("{:?}", files);

    // convert files to repr
    let mut repr: Vec<i32> = vec![-1; orig_repr.len()];
    for file in files.iter() {
        for i in file.start..file.end {
            repr[i] = file.id;
        }
    }
    // println!("{:?}", repr);

    // merge the spaces
    let mut checksum: u64 = 0;
    for i in 0..repr.len() {
        if repr[i] >= 0 {
            checksum += (repr[i] as u64) * (i as u64);
        }
    }
    Some(checksum)
    // println!("Checksum: {}", checksum);

    // for file in files.iter() {
    //     // arithmetic series sum n/2 * (2a + (n-1)d)
    //     checksum += (file.id as u64)
    //         * (file.start as u64 * 2 + (file.size() as u64 - 1) * file.size() as u64)
    //         / 2;
    // }

    // Some(checksum)
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
        assert_eq!(result, Some(2858));
    }
}
