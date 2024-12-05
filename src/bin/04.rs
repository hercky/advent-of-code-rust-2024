advent_of_code::solution!(4);

fn check_xmas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut xmas_count = 0;
    let xmas_chars = ['X', 'M', 'A', 'S'];
    let directions = [
        (1, 0),   // horizontal right
        (-1, 0),  // horizontal left
        (0, 1),   // vertical down
        (0, -1),  // vertical up
        (1, 1),   // diagonal down-right
        (-1, -1), // diagonal up-left
        (1, -1),  // diagonal down-left
        (-1, 1),  // diagonal up-right
    ];

    for &(di, dj) in &directions {
        if (0..4).all(|k| {
            let ni = i as isize + k * di;
            let nj = j as isize + k * dj;
            ni >= 0
                && nj >= 0
                && ni < grid.len() as isize
                && nj < grid[0].len() as isize
                && grid[ni as usize][nj as usize] == xmas_chars[k as usize]
        }) {
            xmas_count += 1;
        }
    }

    xmas_count
}

fn check_mas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> u32 {
    let mut diag_1_ok = false;
    let mut diag_2_ok = false;

    let mas_chars = ['M', 'A', 'S'];

    let diag_1 = [(-1, -1, 1, 1), (1, 1, -1, -1)];
    let diag_2 = [(-1, 1, 1, -1), (1, -1, -1, 1)];

    for &(oi, oj, di, dj) in &diag_1 {
        if (0..3).all(|k| {
            let ni = i as isize + oi + di * k;
            let nj = j as isize + oj + dj * k;
            ni >= 0
                && nj >= 0
                && ni < grid.len() as isize
                && nj < grid[0].len() as isize
                && grid[ni as usize][nj as usize] == mas_chars[k as usize]
        }) {
            diag_1_ok = true;
        }
    }

    for &(oi, oj, di, dj) in &diag_2 {
        if (0..3).all(|k| {
            let ni = i as isize + oi + di * k;
            let nj = j as isize + oj + dj * k;
            ni >= 0
                && nj >= 0
                && ni < grid.len() as isize
                && nj < grid[0].len() as isize
                && grid[ni as usize][nj as usize] == mas_chars[k as usize]
        }) {
            diag_2_ok = true;
        }
    }

    if diag_1_ok && diag_2_ok {
        1
    } else {
        0
    }
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
            if grid[row][col] == 'A' {
                counter += check_mas(&grid, row, col);
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
        assert_eq!(result, Some(9));
    }
}
