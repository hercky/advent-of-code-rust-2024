use std::collections::HashSet;
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
    visited: &mut Vec<Vec<i32>>,
    i: usize,
    j: usize,
    id: i32,
) -> (u32, u32) {
    let mut area = 0;
    let mut perimeter = 0;

    let mut queue = VecDeque::new();
    queue.push_back((i, j));

    while let Some((i, j)) = queue.pop_front() {
        if visited[i][j] >= 0 {
            continue;
        }

        visited[i][j] = id;
        area += 1;

        let all_neighbors = get_neighbors(grid, i, j);
        perimeter += 4 - all_neighbors.len() as u32;

        for (ni, nj) in all_neighbors {
            if visited[ni][nj] == -1 {
                queue.push_back((ni, nj));
            }
        }
    }

    (area, perimeter)
}

#[derive(Debug)]
struct Region {
    id: i32,
    plant: char,
    area: u32,
    perimeter: u32,
}

impl Region {
    fn new(id: i32, plant: char, area: u32, perimeter: Option<u32>) -> Self {
        Self {
            id,
            plant,
            area,
            perimeter: perimeter.unwrap_or(0),
        }
    }

    fn cost(&self) -> u64 {
        self.area as u64 * self.perimeter as u64
    }

    fn set_perimeter(&mut self, perimeter: u32) {
        self.perimeter = perimeter;
    }
}

fn is_corner(padded_visited: &Vec<Vec<i32>>, id: i32, i: i32, j: i32) -> bool {
    let m = padded_visited.len() as i32;
    let n = padded_visited[0].len() as i32;

    let mut same_id = 0;

    if i - 1 >= 0 && j - 1 >= 0 && padded_visited[(i - 1) as usize][(j - 1) as usize] == id {
        same_id += 1;
    }
    if i - 1 >= 0 && j + 1 < n && padded_visited[(i - 1) as usize][(j + 1) as usize] == id {
        same_id += 1;
    }
    if i + 1 < m && j - 1 >= 0 && padded_visited[(i + 1) as usize][(j - 1) as usize] == id {
        same_id += 1;
    }
    if i + 1 < m && j + 1 < n && padded_visited[(i + 1) as usize][(j + 1) as usize] == id {
        same_id += 1;
    }

    same_id == 1 || same_id == 3
}

fn get_corners(visited: &Vec<Vec<i32>>, id: i32) -> u32 {
    let m = visited.len() as i32;
    let n = visited[0].len() as i32;

    let mut padded_visited = vec![vec![-1; (2 * n + 1) as usize]; (2 * m + 1) as usize];

    for i in 0..m as usize {
        for j in 0..n as usize {
            padded_visited[2 * i + 1][2 * j + 1] = visited[i][j];
        }
    }

    let mut corners = 0;

    for i in (0..padded_visited.len()).step_by(2) {
        for j in (0..padded_visited[0].len()).step_by(2) {
            if is_corner(&padded_visited, id, i as i32, j as i32) {
                corners += 1;
            }
        }
    }

    corners
}
pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);

    let mut visited = vec![vec![-1; grid[0].len()]; grid.len()];

    let mut cost: u64 = 0;

    let mut id = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if visited[i][j] == -1 {
                let (area, parameter) = get_regional_values(&grid, &mut visited, i, j, id);
                cost += area as u64 * parameter as u64;
                id += 1;
            }
        }
    }

    Some(cost)
}

pub fn part_two(input: &str) -> Option<u64> {
    let grid = parse_input(input);

    let mut visited = vec![vec![-1; grid[0].len()]; grid.len()];

    let mut cost: u64 = 0;

    let mut id = 0;
    let mut regions: Vec<Region> = Vec::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if visited[i][j] == -1 {
                let (area, _) = get_regional_values(&grid, &mut visited, i, j, id);
                let region = Region::new(id, grid[i][j], area, None);
                regions.push(region);
                id += 1;
            }
        }
    }

    for region in &mut regions {
        // println!("---- Plant {} -------", region.plant);
        region.set_perimeter(get_corners(&visited, region.id));
    }

    for region in &regions {
        // println!("{:?}", region);
    }

    for i in 0..regions.len() {
        cost += regions[i].cost();
    }

    Some(cost)
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
        assert_eq!(result, Some(1206));
    }
}
