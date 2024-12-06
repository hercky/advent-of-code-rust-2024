advent_of_code::solution!(5);
use std::collections::HashMap;

fn floyd_warshall(
    adjacency_matrix: &Vec<Vec<bool>>,
    nodes_in_order: &Vec<usize>,
) -> Vec<Vec<bool>> {
    let n = adjacency_matrix.len();

    let mut paths: Vec<Vec<bool>> = adjacency_matrix.clone();
    let mut sub_adjacency_matrix = adjacency_matrix.clone();

    for i in 0..n {
        if !nodes_in_order.contains(&i) {
            // set the row and column to false
            for j in 0..n {
                sub_adjacency_matrix[i][j] = false;
                sub_adjacency_matrix[j][i] = false;
            }
        }
    }
    // check if there is a path from i to j
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                paths[i][j] |= sub_adjacency_matrix[i][k] & sub_adjacency_matrix[k][j];
            }
        }
    }

    paths
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut node_to_id = HashMap::new();
    let mut id_to_node = HashMap::new();
    let mut edges = Vec::new();

    let mut edge_scan = true;

    let mut orders: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            edge_scan = false;
            continue;
        }

        if edge_scan {
            let (left, right) = line.trim().split_once('|').unwrap();

            let left = left.parse::<u32>().unwrap();
            let right = right.parse::<u32>().unwrap();

            if !node_to_id.contains_key(&left) {
                node_to_id.insert(left, node_to_id.len());
                id_to_node.insert(node_to_id.len() - 1, left);
            }
            if !node_to_id.contains_key(&right) {
                node_to_id.insert(right, node_to_id.len());
                id_to_node.insert(node_to_id.len() - 1, right);
            }
            // add the edge
            edges.push((node_to_id[&left], node_to_id[&right]));
        } else {
            orders.push(
                line.trim()
                    .split(',')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect(),
            );
        }
    }

    let n = node_to_id.len();
    let mut adjacency_matrix: Vec<Vec<bool>> = vec![vec![false; n]; n];

    for (left, right) in edges {
        adjacency_matrix[left][right] = true;
    }

    // Print matrix with headers
    // println!("Adjacency Matrix:");

    // // Print column headers
    // print!("     ");
    // for j in 0..n {
    //     print!("{:4} ", id_to_node[&j]);
    // }
    // println!();

    // // Print rows with headers
    // for i in 0..n {
    //     print!("{:4} ", id_to_node[&i]);
    //     for j in 0..n {
    //         print!("{:4} ", if adjacency_matrix[i][j] { "1" } else { "0" });
    //     }
    //     println!();
    // }
    // println!();

    // prints the paths matrix
    // println!("Paths Matrix:");
    // for i in 0..n {
    //     print!("{:4} ", id_to_node[&i]);
    //     for j in 0..n {
    //         print!("{:4} ", if paths[i][j] { "1" } else { "0" });
    //     }
    //     println!();
    // }
    // println!();
    // println!("{:?}", paths);

    let mut answer = 0;

    for order in orders {
        let mut legit = true;

        let nodes_in_order = order.iter().map(|x| node_to_id[&x]).collect::<Vec<_>>();
        let paths = floyd_warshall(&adjacency_matrix, &nodes_in_order);

        for i in 0..order.len() {
            for j in 0..i {
                if paths[node_to_id[&order[i]]][node_to_id[&order[j]]] {
                    legit = false;
                    break;
                }
            }
            if !legit {
                break;
            }
        }
        if legit {
            answer += order[order.len() / 2];
        }
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut node_to_id = HashMap::new();
    let mut id_to_node = HashMap::new();
    let mut edges = Vec::new();

    let mut edge_scan = true;

    let mut orders: Vec<Vec<u32>> = Vec::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            edge_scan = false;
            continue;
        }

        if edge_scan {
            let (left, right) = line.trim().split_once('|').unwrap();

            let left = left.parse::<u32>().unwrap();
            let right = right.parse::<u32>().unwrap();

            if !node_to_id.contains_key(&left) {
                node_to_id.insert(left, node_to_id.len());
                id_to_node.insert(node_to_id.len() - 1, left);
            }
            if !node_to_id.contains_key(&right) {
                node_to_id.insert(right, node_to_id.len());
                id_to_node.insert(node_to_id.len() - 1, right);
            }
            // add the edge
            edges.push((node_to_id[&left], node_to_id[&right]));
        } else {
            orders.push(
                line.trim()
                    .split(',')
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect(),
            );
        }
    }

    let n = node_to_id.len();
    let mut adjacency_matrix: Vec<Vec<bool>> = vec![vec![false; n]; n];

    for (left, right) in edges {
        adjacency_matrix[left][right] = true;
    }

    // bubble sort on the sub adjacency matrix

    let mut answer = 0;

    for order in orders {
        let nodes_in_order = order.iter().map(|x| node_to_id[&x]).collect::<Vec<_>>();
        let paths = floyd_warshall(&adjacency_matrix, &nodes_in_order);

        let mut legit = true;

        let nodes_in_order = order.iter().map(|x| node_to_id[&x]).collect::<Vec<_>>();
        let paths = floyd_warshall(&adjacency_matrix, &nodes_in_order);

        for i in 0..order.len() {
            for j in 0..i {
                if paths[node_to_id[&order[i]]][node_to_id[&order[j]]] {
                    legit = false;
                    break;
                }
            }
            if !legit {
                break;
            }
        }
        if !legit {
            let mut sorted_order = order.clone();
            // sort the nodes in order based on the preference given by the paths matrix
            sorted_order.sort_by(|a, b| {
                paths[node_to_id[a]][node_to_id[b]].cmp(&paths[node_to_id[b]][node_to_id[a]])
            });
            // println!("{:?}", sorted_order);
            answer += sorted_order[sorted_order.len() / 2];
        }
    }

    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
