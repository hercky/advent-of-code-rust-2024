advent_of_code::solution!(18);

use regex::Regex;
use std::collections::VecDeque;

const MAX_BYTES: usize = 1024;
const MAX_SIZE: usize = 71;

// const MAX_BYTES: usize = 12;
// const MAX_SIZE: usize = 7;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    let re = Regex::new(r"(?P<x>\d+),(?P<y>\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = re.captures(line).unwrap();
            (
                captures["x"].parse().unwrap(),
                captures["y"].parse().unwrap(),
            )
        })
        .collect()
}

fn create_grid(points: &Vec<(usize, usize)>, max_bytes: usize) -> Vec<Vec<u32>> {
    let mut grid = vec![vec![0; MAX_SIZE]; MAX_SIZE];
    for &(x, y) in points.iter().take(max_bytes) {
        grid[y][x] = 1;
    }
    grid
}

fn smallest_path(grid: &Vec<Vec<u32>>, start: (usize, usize), end: (usize, usize)) -> Option<u32> {
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    // let mut path = Vec::new();
    // path.push(start);

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    visited[start.0][start.1] = true;

    while let Some((current, distance)) = queue.pop_front() {
        if current == end {
            return Some(distance);
        }
        // println!("Visiting: {:?}", current);

        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = current.0.checked_add_signed(dx);
            let ny = current.1.checked_add_signed(dy);
            if let (Some(nx), Some(ny)) = (nx, ny) {
                if nx < MAX_SIZE && ny < MAX_SIZE && !visited[nx][ny] && grid[nx][ny] == 0 {
                    queue.push_back(((nx, ny), distance + 1));
                    visited[nx][ny] = true;
                }
            }
        }
    }

    None
}

fn print_path_on_grid(grid: &Vec<Vec<u32>>, path: Vec<(usize, usize)>) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if path.contains(&(i, j)) {
                print!("O");
            } else {
                print!("{}", if cell == 0 { '.' } else { '#' });
            }
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let points = parse_input(input);
    let grid = create_grid(&points, MAX_BYTES);
    println!("================");

    let start = (0, 0);
    let end = (MAX_SIZE - 1, MAX_SIZE - 1);
    let distance = smallest_path(&grid, start, end)?;

    // println!("Distance: {}", path.len());
    // print_path_on_grid(&grid, path);

    Some(distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let points = parse_input(input);

    let start = (0, 0);
    let end = (MAX_SIZE - 1, MAX_SIZE - 1);

    let mut low: usize = MAX_BYTES;
    let mut high: usize = points.len();

    while low < high {
        let mid = (low + high) / 2;
        let new_points = points[0..mid].to_vec();
        let grid = create_grid(&new_points, mid);

        if let Some(distance) = smallest_path(&grid, start, end) {
            low = mid + 1;
        } else {
            high = mid;
        }
    }

    println!("{:?}", points[high - 1]);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
