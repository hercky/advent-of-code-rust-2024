advent_of_code::solution!(16);

use std::collections::{HashSet, VecDeque};

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
    end: (usize, usize),
    start_dir: Direction,
) -> Option<u64> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, start_dir, 0, vec![(start.0, start.1, start_dir)]));

    let mut best_cost = u64::MAX;
    let mut best_path: Vec<(usize, usize, Direction)> = Vec::new();

    while let Some((current_loc, dir, cost, path)) = queue.pop_front() {
        // println!(
        //     "current_loc: {:?}, dir: {:?}, cost: {}",
        //     current_loc, dir, cost
        // );
        // println!("path: {:?}", path);

        visited.insert((current_loc, dir));

        if grid[current_loc.0][current_loc.1] == 'E' {
            println!("found end at {:?} with cost {}", (current_loc, dir), cost);
            if cost < best_cost {
                best_cost = cost;
                best_path = path.clone();
            }
            continue;
        }

        // move forward
        let next_loc = (
            (current_loc.0 as i32 + dir.delta().0) as usize,
            (current_loc.1 as i32 + dir.delta().1) as usize,
        );
        let next_dir = dir;
        // println!(
        //     "Trying to move to next_loc: {:?}, next_dir: {:?}",
        //     next_loc, next_dir
        // );
        if is_valid_move(grid, next_loc) && !visited.contains(&(next_loc, next_dir)) {
            // println!(
            //     "Moving to next_loc: {:?}, next_dir: {:?}",
            //     next_loc, next_dir
            // );
            let mut new_path = path.clone();
            new_path.push((next_loc.0, next_loc.1, dir));
            queue.push_back((next_loc, next_dir, cost + COST_MOVING_FORWARD, new_path));
        }

        // rotate 90 degrees left
        let next_dir = dir.rotate_left();
        if !visited.contains(&(current_loc, next_dir)) {
            let mut new_path = path.clone();
            new_path.push((current_loc.0, current_loc.1, next_dir));
            queue.push_back((
                current_loc,
                next_dir,
                cost + COST_SWITCHING_DIRECTION,
                new_path,
            ));
        }

        // rotate 90 degrees right
        let next_dir = dir.rotate_right();
        if !visited.contains(&(current_loc, next_dir)) {
            let mut new_path = path.clone();
            new_path.push((current_loc.0, current_loc.1, next_dir));
            queue.push_back((
                current_loc,
                next_dir,
                cost + COST_SWITCHING_DIRECTION,
                new_path,
            ));
        }

        // println!("--------------------------------");
        // println!("Queue contents:");
        // for (loc, dir, cost, _) in queue.iter() {
        //     println!(
        //         "  - location: {:?}, direction: {:?}, cost: {}",
        //         loc, dir, cost
        //     );
        // }
    }

    // print_grid_with_visited(&grid, &visited);
    // println!("\nPath taken:");
    // print_grid_with_path(&grid, &best_path);

    if best_cost == u64::MAX {
        None
    } else {
        Some(best_cost)
    }
}

fn print_grid_with_visited(grid: &Vec<Vec<char>>, visited: &HashSet<((usize, usize), Direction)>) {
    for (i, row) in grid.iter().enumerate() {
        for (j, &c) in row.iter().enumerate() {
            if visited.iter().any(|((x, y), _)| *x == i && *y == j) {
                print!("*");
            } else {
                print!("{}", c);
            }
        }
        println!();
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

    let cost = bfs(&grid, start, end, E)?;

    Some(cost as u32)
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
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
