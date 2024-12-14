use std::collections::HashMap;

advent_of_code::solution!(6);

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn get_offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

fn get_position(grid: &Vec<Vec<char>>) -> (usize, usize, Direction) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            match grid[i][j] {
                '^' => return (i, j, Direction::Up),
                '>' => return (i, j, Direction::Right),
                'v' => return (i, j, Direction::Down),
                '<' => return (i, j, Direction::Left),
                _ => continue,
            }
        }
    }
    unreachable!()
}

fn check_blocked(grid: &Vec<Vec<char>>, next_x: i32, next_y: i32) -> bool {
    if grid[next_x as usize][next_y as usize] == '#' {
        return true;
    }

    false
}

fn check_inside(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    x >= 0 && x < grid.len() as i32 && y >= 0 && y < grid[0].len() as i32
}

fn populate_path(grid: &Vec<Vec<char>>) -> (u32, Vec<Vec<char>>) {
    let mut path = grid.clone();

    let mut inside = true;

    while inside {
        // print the grid
        // for row in path.iter() {
        //     println!("{}", row.iter().collect::<String>());
        // }
        // println!("{}", "=".repeat(path[0].len()));

        let (x, y, direction) = get_position(&path);
        let (dx, dy) = direction.get_offset();

        // println!("x: {}, y: {}, direction: {:?}", x, y, direction);
        // println!("{}", "=".repeat(path[0].len()));

        let next_x = x as i32 + dx;
        let next_y = y as i32 + dy;

        if !check_inside(&path, next_x, next_y) {
            inside = false;
            path[x as usize][y as usize] = 'X';
            break;
        }

        if check_blocked(&path, next_x, next_y) {
            // rotate 90 degrees
            let new_direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
            // update the direction
            match new_direction {
                Direction::Up => path[x as usize][y as usize] = '^',
                Direction::Right => path[x as usize][y as usize] = '>',
                Direction::Down => path[x as usize][y as usize] = 'v',
                Direction::Left => path[x as usize][y as usize] = '<',
            }
        } else {
            // mark that we've been here
            path[x as usize][y as usize] = 'X';

            // go to the next
            match direction {
                Direction::Up => path[next_x as usize][next_y as usize] = '^',
                Direction::Right => path[next_x as usize][next_y as usize] = '>',
                Direction::Down => path[next_x as usize][next_y as usize] = 'v',
                Direction::Left => path[next_x as usize][next_y as usize] = '<',
            }
        }
    }

    // count the number of Xs in the path
    let mut count: u32 = 0;
    for row in 0..path.len() {
        for cell in 0..path[row].len() {
            if path[row][cell] == 'X' {
                count += 1;
            }
        }
    }

    (count, path)
}

fn check_loop(grid: &Vec<Vec<char>>) -> bool {
    let mut path = grid.clone();
    let mut direction_map: HashMap<(usize, usize, Direction), bool> = HashMap::new();

    let mut inside = true;
    let mut loop_check: bool = false;

    while inside && !loop_check {
        let (x, y, direction) = get_position(&path);
        let (dx, dy) = direction.get_offset();

        let next_x = x as i32 + dx;
        let next_y = y as i32 + dy;

        if !check_inside(&path, next_x, next_y) {
            return false;
            // inside = false;
            // path[x as usize][y as usize] = 'X';
        }

        if check_blocked(&path, next_x, next_y) {
            // record the direction and the obstacle
            // then we have a loop
            if direction_map.contains_key(&(x, y, direction.clone())) {
                // if for this x,y we have this change of directions
                return true;
            } else {
                direction_map.insert((x, y, direction.clone()), true);
            }

            // rotate 90 degrees
            let new_direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };

            // update the direction
            match new_direction {
                Direction::Up => path[x as usize][y as usize] = '^',
                Direction::Right => path[x as usize][y as usize] = '>',
                Direction::Down => path[x as usize][y as usize] = 'v',
                Direction::Left => path[x as usize][y as usize] = '<',
            }
        } else {
            // mark that we've been here
            path[x as usize][y as usize] = 'X';

            // go to the next
            match direction {
                Direction::Up => path[next_x as usize][next_y as usize] = '^',
                Direction::Right => path[next_x as usize][next_y as usize] = '>',
                Direction::Down => path[next_x as usize][next_y as usize] = 'v',
                Direction::Left => path[next_x as usize][next_y as usize] = '<',
            }
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (count, _) = populate_path(&grid);

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut alt_grid = grid.clone();

    let (_, original_path) = populate_path(&grid);

    let mut count_loop: u32 = 0;

    for i in 0..alt_grid.len() {
        for j in 0..alt_grid[i].len() {
            let total_cells = alt_grid.len() * alt_grid[0].len();
            let current_cell = i * alt_grid[0].len() + j;
            let percentage = (current_cell as f64 / total_cells as f64 * 100.0) as u32;
            println!("Progress: {}%", percentage);

            if alt_grid[i][j] == '.' && original_path[i][j] == 'X' {
                alt_grid[i][j] = '#';
                if check_loop(&alt_grid) {
                    // println!("loop found at: {}, {}", i, j);
                    count_loop += 1;
                }
                alt_grid[i][j] = '.';
            }
        }
    }

    Some(count_loop)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
