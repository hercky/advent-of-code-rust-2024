advent_of_code::solution!(15);

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let mut parts = input.split("\n\n");
    let grid = parts
        .next()
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let path = parts.next().unwrap().trim().chars().collect();

    (grid, path)
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn take_step(grid: &mut Vec<Vec<char>>, robot: (usize, usize), step: char) -> (usize, usize) {
    let (mut rx, mut ry) = robot;
    match step {
        '>' => {
            let mut k = ry + 1;
            while grid[rx][k] == 'O' {
                k += 1;
            }
            if grid[rx][k] == '.' {
                // push the block to the right
                grid[rx][k] = grid[rx][ry + 1];
                grid[rx][ry + 1] = '.';
                // move the robot
                grid[rx][ry + 1] = '@';
                grid[rx][ry] = '.';
                ry = ry + 1;
            }
        }
        '<' => {
            let mut k = ry - 1;
            while grid[rx][k] == 'O' {
                k -= 1;
            }
            if grid[rx][k] == '.' {
                // swap
                grid[rx][k] = grid[rx][ry - 1];
                grid[rx][ry - 1] = '.';
                // // move the robot
                grid[rx][ry - 1] = '@';
                grid[rx][ry] = '.';
                ry = ry - 1;
            }
        }
        'v' => {
            let mut k = rx + 1;
            while grid[k][ry] == 'O' {
                k += 1;
            }
            if grid[k][ry] == '.' {
                // swap
                grid[k][ry] = grid[rx + 1][ry];
                grid[rx + 1][ry] = '.';
                // move the robot
                grid[rx + 1][ry] = '@';
                grid[rx][ry] = '.';

                rx = rx + 1;
            }
        }
        '^' => {
            let mut k = rx - 1;
            while grid[k][ry] == 'O' {
                k -= 1;
            }
            if grid[k][ry] == '.' {
                // swap
                grid[k][ry] = grid[rx - 1][ry];
                grid[rx - 1][ry] = '.';
                // move the robot
                grid[rx - 1][ry] = '@';
                grid[rx][ry] = '.';

                rx = rx - 1;
            }
        }
        _ => {}
    }

    (rx, ry)
}

fn simulate_path(grid: &Vec<Vec<char>>, path: &Vec<char>) -> Vec<Vec<char>> {
    let mut grid = grid.clone();

    // find the start (i,j) of the @ sign
    let mut rx = 0;
    let mut ry = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '@' {
                rx = i;
                ry = j;
                grid[i][j] = '.';
                break;
            }
        }
    }

    for direction in path {
        (rx, ry) = take_step(&mut grid, (rx, ry), *direction);
        // println!("{}", direction);
        // print_grid(&grid);
        // println!("---------------------------");
    }

    grid
}

pub fn part_one(input: &str) -> Option<u64> {
    let (grid, path) = parse_input(input);

    let grid = simulate_path(&grid, &path);

    let mut sum = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'O' {
                sum += i as u64 * 100 + j as u64;
            }
        }
    }

    Some(sum)
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
