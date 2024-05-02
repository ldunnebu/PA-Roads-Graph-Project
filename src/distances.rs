use crate::graph_reader::Graph;
use std::collections::VecDeque;

pub fn avg_shortest(graph: &Graph, start: usize) -> f64 { //BFS
    let mut distances = vec![None; graph.n];
    distances[start] = Some(0);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut count = 0 as usize;
    let mut total_distance = 0 as usize;

    while let Some(current) = queue.pop_front() { //new unprocessed vertex
        if let Some(current_distance) = distances[current] {
            for u in &graph.outedges[current] {
                if distances[*u] == None {
                    let distance = current_distance + 1;
                    distances[*u] = Some(distance);
                    total_distance += distance;
                    count += 1;
                    queue.push_back(*u);
                }
            }
        }   
    }

    if count > 0 {
        total_distance as f64 / count as f64
    } else {
        0.0
    }
}