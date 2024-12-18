advent_of_code::solution!(10);

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn get_neighbors(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();

    if grid[i][j] == 9 {
        return neighbors;
    }

    if i > 0 && (grid[i - 1][j] - grid[i][j] == 1) {
        neighbors.push((i - 1, j));
    }
    if i < grid.len() - 1 && (grid[i + 1][j] - grid[i][j] == 1) {
        neighbors.push((i + 1, j));
    }
    if j > 0 && (grid[i][j - 1] - grid[i][j] == 1) {
        neighbors.push((i, j - 1));
    }
    if j < grid[0].len() - 1 && (grid[i][j + 1] - grid[i][j] == 1) {
        neighbors.push((i, j + 1));
    }

    neighbors
}

fn do_bfs(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> Vec<Vec<bool>> {
    let m = grid.len();
    let n = grid[0].len();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];
    // let mut visited_peaks = HashSet::new();

    let mut queue = Vec::new();
    queue.push((i, j));

    while !queue.is_empty() {
        let (i, j) = queue.remove(0);
        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;

        for (ni, nj) in get_neighbors(grid, i, j) {
            if !visited[ni][nj] {
                queue.push((ni, nj));
            }
        }
    }

    visited
}

fn count_high_points(grid: &Vec<Vec<i32>>, visited: &Vec<Vec<bool>>) -> u32 {
    let m = grid.len();
    let n = grid[0].len();

    let mut ans: u32 = 0;

    for i in 0..m {
        for j in 0..n {
            if visited[i][j] && grid[i][j] == 9 {
                ans += 1;
            }
        }
    }

    ans
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let m = grid.len();
    let n = grid[0].len();

    let mut ans: u32 = 0;

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 {
                let visited: Vec<Vec<bool>> = do_bfs(&grid, i, j);
                ans += count_high_points(&grid, &visited);
            }
        }
    }

    Some(ans)
}

fn calculate_unique_paths_via_bfs(grid: &Vec<Vec<i32>>, i: usize, j: usize) -> Vec<Vec<u32>> {
    let m = grid.len();
    let n = grid[0].len();

    let mut visited: Vec<Vec<u32>> = vec![vec![0; n]; m];

    let mut queue = Vec::new();
    queue.push((i, j));

    while !queue.is_empty() {
        let (i, j) = queue.remove(0);

        visited[i][j] = visited[i][j] + 1;

        for (ni, nj) in get_neighbors(grid, i, j) {
            queue.push((ni, nj));
        }
    }

    visited
}

fn count_unique_paths(grid: &Vec<Vec<i32>>, visited: &Vec<Vec<u32>>) -> u32 {
    let m = grid.len();
    let n = grid[0].len();

    let mut ans: u32 = 0;

    for i in 0..m {
        for j in 0..n {
            if visited[i][j] > 0 && grid[i][j] == 9 {
                ans += visited[i][j];
            }
        }
    }

    ans
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let m = grid.len();
    let n = grid[0].len();

    let mut ans: u32 = 0;

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 0 {
                let visited: Vec<Vec<u32>> = calculate_unique_paths_via_bfs(&grid, i, j);
                ans += count_unique_paths(&grid, &visited);
            }
        }
    }

    Some(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
