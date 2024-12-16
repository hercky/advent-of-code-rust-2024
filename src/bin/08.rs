advent_of_code::solution!(8);

use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash)]
struct Antenna {
    x: i32,
    y: i32,
    id: char,
}

impl Antenna {
    fn new(x: i32, y: i32, id: char) -> Self {
        Self { x, y, id }
    }

    fn offset(&self, other: &Antenna) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }

    fn find_antinode(&self, other: &Antenna) -> Option<(i32, i32)> {
        if self.id != other.id {
            return None;
        }

        if self.x == other.x && self.y == other.y {
            // same antenna
            return None;
        }

        let (dx, dy) = self.offset(other);

        // calculate a point on the other side of this antenna, that lies in the same line of slope as the other antenna
        let antinode = (self.x + dx, self.y + dy);

        Some(antinode)
    }

    fn find_resonant_antinodes(
        &self,
        other: &Antenna,
        grid: &Vec<Vec<char>>,
    ) -> Option<Vec<(i32, i32)>> {
        if self.id != other.id {
            return None;
        }

        if self.x == other.x && self.y == other.y {
            // same antenna
            return None;
        }

        let (dx, dy) = self.offset(other);
        let mut antinodes = Vec::new();

        let mut increment = 0;
        while in_grid(&grid, self.x + dx * increment, self.y + dy * increment) {
            antinodes.push((self.x + dx * increment, self.y + dy * increment));
            increment += 1;
        }

        Some(antinodes)
    }
}

fn in_grid(grid: &Vec<Vec<char>>, x: i32, y: i32) -> bool {
    x >= 0 && x < grid[0].len() as i32 && y >= 0 && y < grid.len() as i32
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antennas = Vec::new();
    // find all antennas
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let c = grid[i][j];
            if c.is_alphanumeric() {
                antennas.push(Antenna::new(i as i32, j as i32, c));
            }
        }
    }

    // solution

    let mut ans = 0;
    let mut unique_antinodes = HashSet::new();

    for antenna in &antennas {
        for other in &antennas {
            if let Some(antinode) = antenna.find_antinode(other) {
                if in_grid(&grid, antinode.0, antinode.1) {
                    unique_antinodes.insert(antinode);
                }
            }
        }
    }

    Some(unique_antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antennas = Vec::new();
    // find all antennas
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let c = grid[i][j];
            if c.is_alphanumeric() {
                antennas.push(Antenna::new(i as i32, j as i32, c));
            }
        }
    }

    // solution
    let mut unique_antinodes = HashSet::new();

    for antenna in &antennas {
        for other in &antennas {
            if let Some(antinodes) = antenna.find_resonant_antinodes(other, &grid) {
                for antinode in antinodes {
                    unique_antinodes.insert(antinode);
                }
            }
        }
    }

    Some(unique_antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
