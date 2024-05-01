mod graph_reader;
mod distances;

//mod pagerank_testofdata;
//use pagerank_testofdata::pagerank;

use graph_reader::{Graph, read_graph};
use distances::avg_shortest;
use rand::distributions::{Distribution, Uniform};
use std::time::Instant;

fn main() {
    //let start = Instant::now();
    let (n, edges) = read_graph("C:/Users/ldunn/DS210/project/code/src/PA_Roads.txt");
    let graph = Graph::create_undirected(n, &edges);
    //let graph_creation_time = start.elapsed();
    //println!("Time taken to create graph: {:?}", graph_creation_time); takes just under a second using cargo run, about half a second using --release

    /*after running the original code with timer and updates, I estimated with the given times that it 
    would take over 8 hours. That being the case, I have taken a random sample of 10,000 vertices of 
    the 1,000,000+ in the dataset*/
    let mut rng = rand::thread_rng();
    let between = Uniform::new(0, n);
    let mut sample = std::collections::HashSet::new();
    for _ in 0..10000 {
        sample.insert(between.sample(&mut rng));
    }

    let start_time = Instant::now();
    let mut av_dist = 0.0;
    let mut count = 0.0;

    for &start in sample.iter() {
        let average_length = avg_shortest(&graph, start);
        av_dist += average_length;
        count += 1.0;
        if (count as usize) % 1000 == 0 {
            let current = start_time.elapsed();
            println!("Average shortest path length from vertex {} is {:.4}, with time taken for {:?} vertices = {:?}", start, average_length, (count as isize), current);
        }
    }

    let overall_average = av_dist / count as f64;
    println!("Overall average shortest path length for 1000 vertices: {:.4}", overall_average);
    println!("Computation time: {:?}", start_time.elapsed()); 
}

// in main
    //let start_vertex = 0;
    //let average_length = graph.avg_shortest(start_vertex);
    //println!("Average shortest path length from vertex {} is {}", start_vertex, average_length);
/* 
    testing data to see if I have read it correctly using prewritten page rank function 
    from homework - tested the time that it took to run just in case my graph creation wasn't 
    the part slowing down the code

    let start = Instant::now();
    pagerank(&graph);
    let pagerank_time = start.elapsed();
    println!("Time taken for PageRank: {:?}", pagerank_time);


DATA
    Nodes	1088092
    Edges	1541898
    Nodes in largest WCC	1087562 (1.000)
    Edges in largest WCC	1541514 (1.000)
    Nodes in largest SCC	1087562 (1.000)
    Edges in largest SCC	1541514 (1.000)
    Average clustering coefficient	0.0465
    Number of triangles	67150
    Fraction of closed triangles	0.02062
    Diameter (longest shortest path)	786
    90-percentile effective diameter	5.3e+02
*/