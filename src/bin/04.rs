advent_of_code::solution!(4);

fn check_xmas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut xmas_count = 0;
    let horizontal_length = grid[i].len();
    let vertical_length = grid.len();

    // check horizontal XMAS pattern both directions
    if i + 3 < horizontal_length {
        if grid[i][j] == 'X'
            && grid[i + 1][j] == 'M'
            && grid[i + 2][j] == 'A'
            && grid[i + 3][j] == 'S'
        {
            xmas_count += 1;
        }
    }

    if (i as i32 - 3) >= 0 {
        if grid[i][j] == 'X'
            && grid[i - 1][j] == 'M'
            && grid[i - 2][j] == 'A'
            && grid[i - 3][j] == 'S'
        {
            xmas_count += 1;
        }
    }

    // vertical XMAS pattern both directions
    if j + 3 < vertical_length {
        if grid[i][j] == 'X'
            && grid[i][j + 1] == 'M'
            && grid[i][j + 2] == 'A'
            && grid[i][j + 3] == 'S'
        {
            xmas_count += 1;
        }
    }

    if (j as i32 - 3) >= 0 {
        if grid[i][j] == 'X'
            && grid[i][j - 1] == 'M'
            && grid[i][j - 2] == 'A'
            && grid[i][j - 3] == 'S'
        {
            xmas_count += 1;
        }
    }

    // diagonal XMAS pattern
    // down and right
    if i + 3 < horizontal_length && j + 3 < vertical_length {
        if grid[i][j] == 'X'
            && grid[i + 1][j + 1] == 'M'
            && grid[i + 2][j + 2] == 'A'
            && grid[i + 3][j + 3] == 'S'
        {
            xmas_count += 1;
        }
    }

    // up and left
    if (i as i32 - 3) >= 0 && (j as i32 - 3) >= 0 {
        if grid[i][j] == 'X'
            && grid[i - 1][j - 1] == 'M'
            && grid[i - 2][j - 2] == 'A'
            && grid[i - 3][j - 3] == 'S'
        {
            xmas_count += 1;
        }
    }

    // up  and right
    if (i as i32 + 3) < horizontal_length as i32 && (j as i32 - 3) >= 0 {
        if grid[i][j] == 'X'
            && grid[i + 1][j - 1] == 'M'
            && grid[i + 2][j - 2] == 'A'
            && grid[i + 3][j - 3] == 'S'
        {
            xmas_count += 1;
        }
    }

    // down and left
    if (i as i32 - 3) >= 0 && (j as i32 + 3) < vertical_length as i32 {
        if grid[i][j] == 'X'
            && grid[i - 1][j + 1] == 'M'
            && grid[i - 2][j + 2] == 'A'
            && grid[i - 3][j + 3] == 'S'
        {
            xmas_count += 1;
        }
    }

    xmas_count
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut counter = 0;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'X' {
                counter += check_xmas(&grid, row, col);
            }
        }
    }

    Some(counter)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut counter = 0;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            if grid[row][col] == 'X' {
                counter += check_xmas(&grid, row, col);
            }
        }
    }

    Some(counter)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
