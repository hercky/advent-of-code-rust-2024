advent_of_code::solution!(24);

use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
enum Op {
    And,
    Or,
    Xor,
    Input,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct Gate {
    name: String,
    op: Op,
    left: Option<String>,
    right: Option<String>,
    output: Option<u8>,
}

impl Gate {
    fn new(
        name: String,
        op: Op,
        left: Option<String>,
        right: Option<String>,
        output: Option<u8>,
    ) -> Self {
        Self {
            name,
            op,
            left,
            right,
            output,
        }
    }
}

fn evaluate(gates: &mut HashMap<String, Gate>, name: &str) -> u8 {
    println!("Evaluating {}", name);
    println!("{:?}", gates.get(name).unwrap());

    if let Some(output) = gates.get(name).unwrap().output {
        return output;
    }

    let (op, left_name, right_name) = {
        let gate = gates.get(name).unwrap();
        (
            gate.op.clone(),
            gate.left.as_ref().unwrap().clone(),
            gate.right.as_ref().unwrap().clone(),
        )
    };
    let left = evaluate(gates, &left_name);
    let right = evaluate(gates, &right_name);

    println!("{} {} {}", left, right, op);

    let result = match op {
        Op::And => left & right,
        Op::Or => left | right,
        Op::Xor => left ^ right,
        Op::Input => panic!("Input gate should not be evaluated"),
    };

    gates.get_mut(name).unwrap().output = Some(result);
    result
}

fn parse_input(input: &str) -> HashMap<String, Gate> {
    let mut gates = HashMap::new();
    for line in input.lines() {
        if let Some((input, output)) = line.split_once("->") {
            let name = output.trim().to_string();
            let gate = if input.contains("AND") {
                let (left, right) = input.split_once("AND").unwrap();
                Gate::new(
                    name.clone(),
                    Op::And,
                    Some(left.trim().to_string()),
                    Some(right.trim().to_string()),
                    None,
                )
            } else if input.contains("XOR") {
                let (left, right) = input.split_once("XOR").unwrap();
                Gate::new(
                    name.clone(),
                    Op::Xor,
                    Some(left.trim().to_string()),
                    Some(right.trim().to_string()),
                    None,
                )
            } else if input.contains("OR") {
                let (left, right) = input.split_once("OR").unwrap();
                Gate::new(
                    name.clone(),
                    Op::Or,
                    Some(left.trim().to_string()),
                    Some(right.trim().to_string()),
                    None,
                )
            } else {
                continue;
            };
            gates.insert(name.clone(), gate);
        }
        if let Some((input, output)) = line.split_once(":") {
            let name = input.trim().to_string();
            let gate = Gate::new(
                name.clone(),
                Op::Input,
                None,
                None,
                Some(output.trim().parse().unwrap()),
            );
            gates.insert(name, gate);
        }
    }
    gates
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut gates = parse_input(input);

    println!("{:?}", gates);

    let mut ans1 = 0u64;
    for i in 0..64 {
        let name = format!("z{:0>2}", i);
        if !gates.contains_key(&name) {
            break;
        }
        let bit = evaluate(&mut gates, &name);
        ans1 |= u64::from(bit) << i;
    }

    Some(ans1)
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
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
