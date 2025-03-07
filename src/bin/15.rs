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

#[derive(Debug, Clone, Copy)]
enum BlockType {
    Empty,
    Wall,
    Box,
    Robot,
}

impl std::fmt::Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockType::Empty => write!(f, ".."),
            BlockType::Wall => write!(f, "##"),
            BlockType::Box => write!(f, "[]"),
            BlockType::Robot => write!(f, "@."),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Block {
    x_left: u32,
    x_right: u32,
    y: u32,
    block_type: BlockType,
}

impl Block {
    fn new(x: u32, y: u32, block_type: BlockType) -> Self {
        Block {
            x_left: x,
            x_right: x + 1,
            y,
            block_type,
        }
    }
}

fn convert_block_grid(grid: &Vec<Vec<Block>>) -> Vec<Vec<char>> {
    grid.iter()
        .map(|row| {
            row.iter()
                .flat_map(|block| block.block_type.to_string().chars().collect::<Vec<_>>())
                .collect()
        })
        .collect()
}

// fn next_position(direction: char, current_block: Block, grid: &Vec<Vec<char>>) -> (u32, u32, u32) {
//     let (mut x_left, mut x_right, mut y) = (current_block.x_left, current_block.x_right, current_block.y);

//     match direction {
//         '>' => {
//             x_right += 2;
//             x_left += 2;
//         }
//         '<' => {
//             x_right -= 2;
//             x_left -= 2;
//         }
//         'v' => y += 1,
//         '^' => y -= 1,
//     }

//     if grid[x_left][y] == '#' || grid[x_right][y] == '#' {
//         return (x_left, x_right, y, BlockType::Wall);
//     } else if grid[x_left][y] == '.' || grid[x_right][y] == '.' {
//         return (x_left, x_right, y, BlockType::Empty);
//     } else if grid[x_left][y] == ['[', ']'] || grid[x_right][y] == ['[', ']'] {
//         return (x_left, x_right, y, BlockType::Box);
//     }

//     (x_left, x_right, y)
// }

// fn push_block(block: Block, direction: char, grid: &Vec<Vec<char>>) {
//     let (mut x_left, mut x_right, mut y) = (block.x_left, block.x_right, block.y);

//     let next_position

//     if next_position(direction, x_left, x_right, y) == '.' {
//         // make the change for this block
//         // move this block
//     } else if next_position(direction, x_left, x_right, y) == ['[', ']'] {
//         // push that block in the direction
//         push_block(
//             next_position(direction, x_left, x_right, y),
//             direction,
//             grid,
//         );
//     } else {
//         // '#'
//         // dead end, can't move
//     }
// }

fn simulate_path_blocks(grid: &Vec<Vec<char>>, path: &Vec<char>) -> Vec<Vec<char>> {
    let mut grid = grid.clone();

    grid
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid, path) = parse_input(input);

    // convert the grid to a block grid
    let mut block_grid = Vec::new();
    for i in 0..grid.len() {
        let mut row = Vec::new();
        for j in 0..grid[i].len() {
            match grid[i][j] {
                '.' => row.push(Block::new(2 * i as u32, j as u32, BlockType::Empty)),
                '#' => row.push(Block::new(2 * i as u32, j as u32, BlockType::Wall)),
                'O' => row.push(Block::new(2 * i as u32, j as u32, BlockType::Box)),
                '@' => row.push(Block::new(2 * i as u32, j as u32, BlockType::Robot)),
                '\n' => continue,
                _ => panic!("Invalid character in grid"),
            }
        }
        block_grid.push(row);
    }

    let grid = convert_block_grid(&block_grid);

    print_grid(&grid);

    return None;

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
