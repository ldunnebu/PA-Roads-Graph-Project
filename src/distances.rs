use crate::graph_reader::Graph;
use std::collections::VecDeque;

pub fn avg_shortest(graph: &Graph, start: usize) -> f64 {
    let mut queue = VecDeque::new();
    let mut distances = vec![-1; graph.n];
    distances[start] = 0;
    queue.push_back(start);

    let mut count = 0usize;
    let mut total_distance = 0usize;

    while let Some(current) = queue.pop_front() {
        for &neighbor in &graph.outedges[current] {
            if distances[neighbor] == -1 {
                distances[neighbor] = distances[current] + 1;
                total_distance += distances[neighbor] as usize;
                count += 1;
                queue.push_back(neighbor);
            }
        }
    }

    if count > 0 {
        total_distance as f64 / count as f64
    } else {
        0.0
    }
}