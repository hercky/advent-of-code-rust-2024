advent_of_code::solution!(23);

use itertools::Itertools;
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

fn bron_kerbosch<'a>(
    graph: &'a HashMap<&'a str, Vec<&'a str>>,
    r: &mut HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
) -> HashSet<&'a str> {
    if p.is_empty() && x.is_empty() {
        return r.clone();
    }

    let mut best = HashSet::new();
    for v in p.clone() {
        let mut new_r = r.clone();
        new_r.insert(v);

        // Create new sets using intersection with neighbors of v
        let neighbors: HashSet<&str> = graph[v].iter().copied().collect();
        let mut new_p = p.intersection(&neighbors).copied().collect();
        let mut new_x = x.intersection(&neighbors).copied().collect();

        let result = bron_kerbosch(graph, &mut new_r, &mut new_p, &mut new_x);
        if result.len() > best.len() {
            best = result;
        }

        p.remove(v);
        x.insert(v);
    }
    best
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
    let graph = parse_graph(input);

    // Initialize sets for Bron-Kerbosch algorithm
    let mut r = HashSet::new();
    let mut p: HashSet<&str> = graph.keys().copied().collect();
    let mut x = HashSet::new();

    // Find the maximum clique
    let max_clique = bron_kerbosch(&graph, &mut r, &mut p, &mut x);

    println!("{:?}", max_clique.iter().sorted().join(","));

    // Return the size of the maximum clique
    Some(max_clique.len() as u32)
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
        assert_eq!(result, Some(4));
    }
}
