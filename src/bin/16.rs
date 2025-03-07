advent_of_code::solution!(16);

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    N,
    S,
    E,
    W,
}

const COST_SWITCHING_DIRECTION: u64 = 1000;
const COST_MOVING_FORWARD: u64 = 1;

use Direction::*;

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            N => (-1, 0),
            S => (1, 0),
            W => (0, -1),
            E => (0, 1),
        }
    }

    fn rotate_left(&self) -> Self {
        match self {
            N => W,
            S => E,
            E => N,
            W => S,
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            N => E,
            S => W,
            E => S,
            W => N,
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_cell(grid: &Vec<Vec<char>>, cell: char) -> Option<(usize, usize)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if c == cell {
                return Some((i, j));
            }
        }
    }
    None
}

fn is_valid_move(grid: &Vec<Vec<char>>, to: (usize, usize)) -> bool {
    if grid[to.0][to.1] == '#' {
        return false;
    }
    true
}

fn bfs(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    start_dir: Direction,
) -> Option<(u64, Vec<(Vec<(usize, usize, Direction)>, u64)>)> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, start_dir, 0, vec![(start.0, start.1, start_dir)]));
    visited.insert((start, start_dir), 0);
    let mut best_cost = u64::MAX;
    let mut best_path: Vec<(usize, usize, Direction)> = Vec::new();
    let mut all_paths: Vec<(Vec<(usize, usize, Direction)>, u64)> = Vec::new();

    while let Some((current_loc, dir, cost, path)) = queue.pop_front() {
        if grid[current_loc.0][current_loc.1] == 'E' {
            if cost < best_cost {
                best_cost = cost;
                best_path = path.clone();
            }

            all_paths.push((path, cost));
            continue;
        }

        // move forward
        let next_loc = (
            (current_loc.0 as i32 + dir.delta().0) as usize,
            (current_loc.1 as i32 + dir.delta().1) as usize,
        );
        let next_dir = dir;
        let next_cost = cost + COST_MOVING_FORWARD;
        if is_valid_move(grid, next_loc)
            || (visited.contains_key(&(next_loc, next_dir))
                && visited[&(next_loc, next_dir)] >= next_cost)
        {
            let mut new_path = path.clone();
            new_path.push((next_loc.0, next_loc.1, dir));
            queue.push_back((next_loc, next_dir, next_cost, new_path));
            visited.insert((next_loc, next_dir), next_cost);
        }

        // rotate 90 degrees left
        let next_loc = current_loc;
        let next_dir = dir.rotate_left();
        let next_cost = cost + COST_SWITCHING_DIRECTION;
        if !visited.contains_key(&(next_loc, next_dir))
            || visited[&(next_loc, next_dir)] >= next_cost
        {
            let mut new_path = path.clone();
            new_path.push((next_loc.0, next_loc.1, next_dir));
            queue.push_back((next_loc, next_dir, next_cost, new_path));
            visited.insert((next_loc, next_dir), next_cost);
        }

        // rotate 90 degrees right
        let next_loc = current_loc;
        let next_dir = dir.rotate_right();
        let next_cost = cost + COST_SWITCHING_DIRECTION;
        if !visited.contains_key(&(next_loc, next_dir))
            || visited[&(next_loc, next_dir)] >= next_cost
        {
            let mut new_path = path.clone();
            new_path.push((next_loc.0, next_loc.1, next_dir));
            queue.push_back((next_loc, next_dir, next_cost, new_path));
            visited.insert((next_loc, next_dir), next_cost);
        }
    }

    if best_cost == u64::MAX {
        None
    } else {
        Some((best_cost, all_paths))
    }
}

fn print_grid_with_path(grid: &Vec<Vec<char>>, path: &Vec<(usize, usize, Direction)>) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if let Some(pos) = path.iter().position(|(x, y, _)| *x == i && *y == j) {
                let (_, _, dir) = path[pos];
                print!(
                    "{}",
                    match dir {
                        N => '^',
                        S => 'v',
                        W => '<',
                        E => '>',
                    }
                );
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = parse_input(input);

    let start = find_cell(&grid, 'S')?;
    let end = find_cell(&grid, 'E')?;

    // replace S with .
    grid[start.0][start.1] = '.';

    let (cost, _) = bfs(&grid, start, E)?;

    Some(cost as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = parse_input(input);

    let start = find_cell(&grid, 'S')?;
    let end = find_cell(&grid, 'E')?;

    // replace S with .
    grid[start.0][start.1] = '.';

    let (best_cost, all_paths) = bfs(&grid, start, E)?;

    let mut unique_tiles: HashSet<(usize, usize)> = HashSet::new();

    for (path, cost) in all_paths {
        if cost == best_cost {
            print_grid_with_path(&grid, &path);
            for (x, y, _) in path {
                unique_tiles.insert((x, y));
            }
        }
    }

    // println!("unique tiles: {}", unique_tiles.len());
    Some(unique_tiles.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
