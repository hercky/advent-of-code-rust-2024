advent_of_code::solution!(23);

use regex::Regex;
use std::collections::{HashMap, HashSet};

fn parse_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let re = Regex::new(r"(\w+)-(\w+)").unwrap();
    let mut graph = HashMap::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let a = caps.get(1).unwrap().as_str();
        let b = caps.get(2).unwrap().as_str();
        graph.entry(a).or_insert(vec![]).push(b);
        graph.entry(b).or_insert(vec![]).push(a);
        // println!("{:?} {:?}", a, b);
    }
    graph
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = parse_graph(input);

    let mut candidates = Vec::new();
    for node in graph.keys() {
        if node.starts_with('t') {
            candidates.push(node);
        }
    }

    let mut t_triples = HashSet::new();
    // println!("{:?}", candidates.len());

    for t in candidates {
        for a in graph[t].iter() {
            for b in graph[a].iter() {
                if graph[t].contains(b) {
                    let mut triple = vec![t, a, b];
                    triple.sort();
                    t_triples.insert((triple[0], triple[1], triple[2]));
                    // println!("{:?} {:?} {:?}", t, a, b);
                }
            }
        }
    }
    Some(t_triples.len() as u32)
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
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
