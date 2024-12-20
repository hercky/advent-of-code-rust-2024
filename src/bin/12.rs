use std::collections::VecDeque;

advent_of_code::solution!(12);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn get_neighbors(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let m = grid.len();
    let n = grid[0].len();

    if i > 0 && grid[i - 1][j] == grid[i][j] {
        neighbors.push((i - 1, j));
    }
    if i < m - 1 && grid[i + 1][j] == grid[i][j] {
        neighbors.push((i + 1, j));
    }
    if j > 0 && grid[i][j - 1] == grid[i][j] {
        neighbors.push((i, j - 1));
    }
    if j < n - 1 && grid[i][j + 1] == grid[i][j] {
        neighbors.push((i, j + 1));
    }

    neighbors
}

fn get_regional_values(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    i: usize,
    j: usize,
) -> (u32, u32) {
    let mut area = 0;
    let mut perimeter = 0;

    let mut queue = VecDeque::new();
    queue.push_back((i, j));

    while let Some((i, j)) = queue.pop_front() {
        if visited[i][j] {
            continue;
        }

        visited[i][j] = true;
        area += 1;

        let all_neighbors = get_neighbors(grid, i, j);
        perimeter += 4 - all_neighbors.len() as u32;

        for (ni, nj) in all_neighbors {
            if !visited[ni][nj] {
                queue.push_back((ni, nj));
            }
        }
    }

    (area, perimeter)
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);

    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    let mut cost: u64 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if !visited[i][j] {
                let (area, parameter) = get_regional_values(&grid, &mut visited, i, j);
                cost += area as u64 * parameter as u64;
            }
        }
    }

    Some(cost)
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
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
