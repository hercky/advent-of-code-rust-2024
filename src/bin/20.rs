advent_of_code::solution!(20);

use std::collections::HashMap;
use std::collections::VecDeque;

const TIME_LIMIT: usize = 100;
// const TIME_LIMIT: usize = 60;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_coords(grid: &Vec<Vec<char>>, c: char) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == c {
                return Some((i, j));
            }
        }
    }
    None
}

fn bfs(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    end: (usize, usize),
) -> (HashMap<(usize, usize), usize>, Vec<(usize, usize)>) {
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut distance = HashMap::new();
    distance.insert(start, 0);

    let mut path = Vec::new();
    path.push(start);

    while let Some(current) = queue.pop_front() {
        if current == end {
            return (distance, path);
        }

        for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next = (
                (current.0 as isize + direction.0) as usize,
                (current.1 as isize + direction.1) as usize,
            );
            if next.0 < 0 || next.1 < 0 || next.0 >= grid.len() || next.1 >= grid[0].len() {
                continue;
            }
            if grid[next.0][next.1] == '#' {
                continue;
            }
            if distance.contains_key(&next) && distance[&next] <= distance[&current] + 1 {
                continue;
            }
            distance.insert(next, distance[&current] + 1);
            path.push(next);
            queue.push_back(next);
        }
    }

    (distance, path)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let start = get_coords(&grid, 'S').unwrap();
    let end = get_coords(&grid, 'E').unwrap();

    let (start_distance, start_path) = bfs(&grid, start, end);
    let (end_distance, end_path) = bfs(&grid, end, start);

    let mut answer = 0;

    for (k, v) in &start_distance {
        let (x, y) = *k;

        for direction in [(-2, 0), (2, 0), (0, -2), (0, 2)] {
            let next = (
                (x as isize + direction.0) as usize,
                (y as isize + direction.1) as usize,
            );
            if next.0 < 0
                || next.1 < 0
                || next.0 >= grid.len()
                || next.1 >= grid[0].len()
                || !start_distance.contains_key(&next)
            {
                continue;
            }

            // println!("{} {}", start_distance[&next], *v);

            if start_distance[&next] as i32 - *v as i32 >= (TIME_LIMIT + 2) as i32 {
                answer += 1;
            }
        }
    }

    Some(answer)
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
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}