use crate::distances::avg_shortest;
use crate::Graph;
use rand::distributions::{Distribution, Uniform};
use std::time::Instant;
use std::collections::HashSet;

pub fn bfs_sample(graph: &Graph, sample_size: usize, total_size: usize) {
    let mut rng = rand::thread_rng();
    let between = Uniform::new(0, total_size);
    let mut sample = HashSet::new();
    for _ in 0..total_size { //taking a random sample of "sample_size" vertices of the 1,000,000+ in the dataset by randomly scrambling the nodes into a HashSet
        let random_vertex = between.sample(&mut rng);
        sample.insert(random_vertex);
    }

    let start_time = Instant::now();
    let mut total = 0.0;
    let mut count = 0;

    let mut mx_length = 0;
    let mut mx_path = None;

    for u in sample.iter() {
        if count == sample_size { //running until sample_size amount of vertices of a value not equal to 0.0 have been ran
            break;
        }
        else {
            let (average_length, vertex_mx_length, vertex_mx_path, _, _) = avg_shortest(&graph, *u);
            if average_length == 0.0 { //excluding values where the average length returns 0.0 to eliminate outliers and gaps in the data
                continue;
            } 
            else {
                total += average_length;
                count += 1;
                if (count as usize) % (sample_size / 5) == 0 {
                    let current = start_time.elapsed();
                    println!("Average shortest path length currently is {:?}, with time taken for {:?} vertices = {:?}", total / count as f64, (count as isize), current);
                }
                //stats
                if vertex_mx_length > mx_length {
                    mx_length = vertex_mx_length;
                    mx_path = vertex_mx_path;
                }
            }
        }
    }

    let overall_average = total / count as f64;
    println!("Average shortest path length for {:?} vertices: {:?}", count as isize, overall_average); 
    println!("\nSample Statistics (not entire graph):\nMaximum length of Shortest Path: {}\nPath of longest shortest path (start node, end node): {:?}", mx_length, mx_path.unwrap());
    println!("\nComputation time: {:?}", start_time.elapsed());
}