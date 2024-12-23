advent_of_code::solution!(14);

use colored::*;
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::Write;

#[derive(Clone)]
struct Robot {
    x: u32,
    y: u32,
    vx: i32,
    vy: i32,
}

impl Robot {
    fn new(x: u32, y: u32, vx: i32, vy: i32) -> Self {
        Self { x, y, vx, vy }
    }

    fn step(&mut self, multiplier: i32, max_x: u32, max_y: u32) {
        let new_x = (self.x as i32 + self.vx * multiplier).rem_euclid(max_x as i32 + 1);
        let new_y = (self.y as i32 + self.vy * multiplier).rem_euclid(max_y as i32 + 1);
        self.x = new_x as u32;
        self.y = new_y as u32;
    }
}

fn parse_line(line: &str) -> Robot {
    let re = Regex::new(r"p=(?<x>\d+),(?<y>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
    let cap = re.captures(line).unwrap();
    Robot::new(
        cap["x"].parse().unwrap(),
        cap["y"].parse().unwrap(),
        cap["vx"].parse().unwrap(),
        cap["vy"].parse().unwrap(),
    )
}

fn calculate_num_robots_in_quadrants(
    robots: &[Robot],
    max_x: u32,
    max_y: u32,
) -> (u32, u32, u32, u32) {
    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    let mid_x = max_x / 2;
    let mid_y = max_y / 2;

    for robot in robots {
        if robot.x == mid_x || robot.y == mid_y {
            continue;
        }

        if robot.x < mid_x && robot.y < mid_y {
            q1 += 1;
        } else if robot.x < mid_x && robot.y > mid_y {
            q2 += 1;
        } else if robot.x > max_x / 2 && robot.y < max_y / 2 {
            q3 += 1;
        } else if robot.x > max_x / 2 && robot.y > max_y / 2 {
            q4 += 1;
        }
    }

    (q1, q2, q3, q4)
}

fn print_robots_on_grid(robots: &[Robot], max_x: u32, max_y: u32) {
    let mut robots_loc = HashMap::new();

    for robot in robots {
        *robots_loc.entry((robot.x, robot.y)).or_insert(0) += 1;
    }

    for y in 0..max_y {
        for x in 0..max_x {
            if x == max_x / 2 || y == max_y / 2 {
                print!("+");
            } else {
                if robots_loc.contains_key(&(x, y)) {
                    print!("{}", robots_loc[&(x, y)]);
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
}

fn pretty_print_robots_on_grid(
    robots: &[Robot],
    max_x: u32,
    max_y: u32,
    time: i32,
    file_name: &str,
) {
    let mut grid = vec![vec!['.'; max_x as usize + 1]; max_y as usize + 1];

    for robot in robots {
        grid[robot.y as usize][robot.x as usize] = '#';
    }

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_name)
        .unwrap();

    writeln!(file, "{}", time).unwrap();
    for row in grid {
        writeln!(file, "{}", row.iter().collect::<String>()).unwrap();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut robots = input
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();

    let original_robots = robots.clone();

    // problem constraints
    let max_x = 101 - 1;
    let max_y = 103 - 1;

    // test-case constraints
    // let max_x = 11 - 1;
    // let max_y = 7 - 1;

    robots
        .iter_mut()
        .for_each(|robot| robot.step(100, max_x, max_y));

    let mut answer = 0;

    let (q1, q2, q3, q4) = calculate_num_robots_in_quadrants(&robots, max_x, max_y);

    answer = q1 * q2 * q3 * q4;

    print_robots_on_grid(&robots, max_x, max_y);

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = input
        .lines()
        .map(|line| parse_line(line))
        .collect::<Vec<_>>();

    let original_robots = robots.clone();

    // problem constraints
    let max_x = 101 - 1;
    let max_y = 103 - 1;

    for i in 0..10_000 {
        robots
            .iter_mut()
            .for_each(|robot| robot.step(1, max_x, max_y));

        pretty_print_robots_on_grid(&robots, max_x, max_y, i, "output.txt");
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
