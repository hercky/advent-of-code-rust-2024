advent_of_code::solution!(13);

#[derive(Clone)]
pub struct Coordinates {
    x: u64,
    y: u64,
}

impl Coordinates {
    pub fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
}

#[derive(Clone)]
pub struct Location {
    coordinates: Coordinates,
    num_a_token: u64,
    num_b_token: u64,
}

impl Location {
    pub fn new(coordinates: Coordinates, num_a_token: u64, num_b_token: u64) -> Self {
        Self {
            coordinates,
            num_a_token,
            num_b_token,
        }
    }
}

fn solve_for_one_machine(a: Coordinates, b: Coordinates, prize: Coordinates) -> u64 {
    let mut best_answer: Vec<u64> = Vec::new();

    for i in 0..100 {
        for j in 0..100 {
            if a.x * i as u64 + b.x * j as u64 == prize.x
                && a.y * i as u64 + b.y * j as u64 == prize.y
            {
                best_answer.push(3 * i as u64 + j as u64);
            }
        }
    }

    if best_answer.is_empty() {
        0
    } else {
        *best_answer.iter().min().unwrap()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut answer: u32 = 0;

    let mut machine_number = 0;

    input
        .split("\n\n") // Split on double newlines to get groups
        .filter(|group| !group.is_empty())
        .for_each(|group| {
            let mut lines = group.lines();

            // Parse Button A
            let button_a = lines.next().unwrap();
            let (a_x, a_y) = parse_button(button_a, "Button A:");

            // Parse Button B
            let button_b = lines.next().unwrap();
            let (b_x, b_y) = parse_button(button_b, "Button B:");

            // Parse Prize
            let prize = lines.next().unwrap();
            let (prize_x, prize_y) = parse_prize(prize);

            // solve for the prize now !!
            let tokens_for_this_machine = solve_for_one_machine(
                Coordinates::new(a_x, a_y),
                Coordinates::new(b_x, b_y),
                Coordinates::new(prize_x, prize_y),
            );

            machine_number += 1;

            answer += tokens_for_this_machine as u32;
        });

    Some(answer)
}

fn parse_button(line: &str, prefix: &str) -> (u64, u64) {
    let coords = line.strip_prefix(prefix).unwrap().trim();
    let mut parts = coords.split(", ");
    let x = parts
        .next()
        .unwrap()
        .trim_start_matches("X+")
        .parse()
        .unwrap();
    let y = parts
        .next()
        .unwrap()
        .trim_start_matches("Y+")
        .parse()
        .unwrap();
    (x, y)
}

fn parse_prize(line: &str) -> (u64, u64) {
    let coords = line.strip_prefix("Prize:").unwrap().trim();
    let mut parts = coords.split(", ");
    let x = parts
        .next()
        .unwrap()
        .trim_start_matches("X=")
        .parse()
        .unwrap();
    let y = parts
        .next()
        .unwrap()
        .trim_start_matches("Y=")
        .parse()
        .unwrap();
    (x, y)
}

fn manual_solve_linear_equation(a: Coordinates, b: Coordinates, prize: Coordinates) -> u64 {
    let ax = a.x as i64;
    let ay = a.y as i64;
    let bx = b.x as i64;
    let by = b.y as i64;

    let px = prize.x as i64;
    let py = prize.y as i64;

    let b = (px * ay - py * ax) / (ay * bx - ax * by);
    let a = (px * by - py * bx) / (by * ax - ay * bx);

    // if a < 0 || b < 0 {
    //     return 0;
    // }

    if ax * a + bx * b == px && ay * a + by * b == py {
        return (3 * a + b) as u64;
    } else {
        return 0;
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut answer: u64 = 0;

    let mut machine_number = 0;

    input
        .split("\n\n") // Split on double newlines to get groups
        .filter(|group| !group.is_empty())
        .for_each(|group| {
            let mut lines = group.lines();

            // Parse Button A
            let button_a = lines.next().unwrap();
            let (a_x, a_y) = parse_button(button_a, "Button A:");

            // Parse Button B
            let button_b = lines.next().unwrap();
            let (b_x, b_y) = parse_button(button_b, "Button B:");

            // Parse Prize
            let prize = lines.next().unwrap();
            let (mut prize_x, mut prize_y) = parse_prize(prize);

            prize_x = prize_x + 10000000000000;
            prize_y = prize_y + 10000000000000;

            // solve for the prize now !!
            let tokens_for_this_machine = manual_solve_linear_equation(
                Coordinates::new(a_x, a_y),
                Coordinates::new(b_x, b_y),
                Coordinates::new(prize_x, prize_y),
            );

            machine_number += 1;

            answer += tokens_for_this_machine;
        });

    println!("{}", answer);
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
