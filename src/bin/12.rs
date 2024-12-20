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

fn get_perimeter(visited: &Vec<Vec<i32>>, id: i32) -> u32 {
    let mut perimeter = 0;

    let mut x_set: HashSet<i32> = HashSet::new();
    let mut y_set: HashSet<i32> = HashSet::new();

    for i in 0..visited.len() {
        for j in 0..visited[i].len() {
            if visited[i][j] == id {
                // insert all the sides for this cell into the sets
                if i == 0 || i > 0 && visited[i - 1][j] != id {
                    x_set.insert(i as i32 - 1);
                }
                if i == visited.len() - 1 || i < visited.len() - 1 && visited[i + 1][j] != id {
                    x_set.insert(i as i32 + 1);
                }
                if j == 0 || j > 0 && visited[i][j - 1] != id {
                    y_set.insert(j as i32 - 1);
                }
                if j == visited[i].len() - 1 || j < visited[i].len() - 1 && visited[i][j + 1] != id
                {
                    y_set.insert(j as i32 + 1);
                }
            }
        }
    }

    // count unique x and y values
    perimeter += x_set.len() + y_set.len();

    perimeter as u32
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
        region.set_perimeter(get_perimeter(&visited, region.id));
    }

    println!("{:?}", regions);

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
