use crate::graph_reader::Graph;
use std::collections::{HashMap, VecDeque};

pub fn avg_shortest(graph: &Graph, start: usize) -> (f64, usize, Option<(usize, usize)>, usize, usize) { //BFS and stats
    let mut distances = vec![None; graph.n];
    distances[start] = Some(0);

    let mut parent = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut count = 0 as usize;
    let mut total_distance = 0 as usize;

    let mut max_length = 0;
    let mut max_path = None;
    //For test
    let mut min_length = 786; //graph website said diameter of the function is 786
    let mut min_node = start;

    while let Some(current) = queue.pop_front() { //new unprocessed vertex
        if let Some(current_distance) = distances[current] {
            for u in &graph.outedges[current] {
                if distances[*u] == None {
                    let distance = current_distance + 1;
                    distances[*u] = Some(distance);
                    parent.insert(*u, current); //keeping track of the traversed edges
                    total_distance += distance;
                    count += 1;
                    queue.push_back(*u);

                    if distance > max_length {
                        max_length = distance;
                        max_path = Some((start, *u));
                    }
                    if distance < min_length {
                        min_length = distance;
                        min_node = *u;
                    }
                }    
            }
        }   
    }

    if let Some((n1, n2)) = max_path { //working backwards from end node to find 
        let mut path = Vec::new();
        let mut node = n2;
        
        while node != n1 { //work back to find start node
            path.push(node);
            node = parent[&node];
        }
        path.push(n1); //add the start node at the end
        path.reverse(); //it's backwards, turn it around

        let first = path.first().unwrap();
        let last = path.last().unwrap();
        max_path = Some((*first, *last));
    }

    if count > 0 {
        (total_distance as f64 / count as f64, max_length, max_path, min_length, min_node)
    } else {
        (0.0, max_length, None, min_length, min_node)
    }
}