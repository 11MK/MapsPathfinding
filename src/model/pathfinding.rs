use crate::model::{structs::graph::Graph, structs::point::Point};
use crate::model::util::*;

use std::collections::BinaryHeap;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Clone)]
struct State {
    id: usize,
    f_cost: f64,
    g_cost: f64,
}

impl Eq for State {}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .f_cost
            .partial_cmp(&self.f_cost)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

// Backtrack through nodes to recover path, return vec[Points]
fn reconstruct_path(graph: &Graph, came_from: HashMap<usize, usize>, current: usize) -> Vec<Point> {
    let mut total_path = vec![graph.nodes[&current].position];
    let mut node = current;
    while came_from.contains_key(&node) {
        node = came_from[&node];
        total_path.push(graph.nodes[&node].position);
    }
    total_path.reverse();
    total_path
}

// Pathfinding Algorithm,
fn a_star(graph: &Graph, start: Point, goal: Point) -> Option<Vec<Point>> {
    let st_node = graph.find_nearest(start);
    let gl_node = graph.find_nearest(goal);
    let mut came_from = HashMap::new();
    let mut g_score: HashMap<usize, f64> = HashMap::new();
    let mut open_set = BinaryHeap::new();
    open_set.push(State {
        id: st_node.id,
        f_cost: calculate_distance(&start, &goal),
        g_cost: 0.,
    });
    g_score.insert(st_node.id, 0.0);
    println!("{:?}, {:?}", st_node.id, gl_node.id);
    while let Some(State { id, .. }) = open_set.pop() {
        if id == gl_node.id {
            return Some(reconstruct_path(graph, came_from, id));
        }
        for &(distance, neighbor) in graph.get_neighbors(id).unwrap_or(&vec![]) {
            println!("{:?}", open_set.len());
            let tentative_g_score = g_score[&id] + distance;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&f64::MAX) {
                came_from.insert(neighbor, id);
                g_score.insert(neighbor, tentative_g_score);
                let f_score =
                    tentative_g_score + calculate_distance(&graph.nodes[&neighbor].position, &goal);
                open_set.push(State {
                    id: neighbor,
                    f_cost: f_score,
                    g_cost: tentative_g_score,
                });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // read ./data.xml into str
        let data = include_str!("../../data3.xml");
        // let data = std::fs::read_to_string("./data.xml").expect("unable to read file");
        let graph = parse_xml(data);
        // let start = Point { lat: 45.6762759, lon: -111.042363 };
        // let goal = Point { lat: 45.6723792, lon: -111.0479298 };
        let start = Point {
            lat: 45.6408383,
            lon: -111.0372238,
        };
        let goal = Point {
            lat: 45.6480019,
            lon: -111.0377540,
        };
        let path = a_star(&graph, start, goal);
        // print path
        match path {
            Some(p) => {
                for point in p {
                    println!("lat: {}, lon: {}", point.lat, point.lon);
                }
            }
            None => println!("No path found"),
        }

        // assert_eq!(1, Solution::balanced_string("QQWR".to_string()))
        // assert_eq!("bb", Solution::longest_palindrome("cbbd".to_string()));
        // assert_eq!("bab", Solution::longest_palindrome("babad".to_string()));
    }
}
