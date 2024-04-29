mod graph_reader;
//mod pagerank_testofdata;

use graph_reader::{Graph, ListOfEdges, read_graph};
//use pagerank_testofdata::pagerank;
//use std::time::Instant;

fn main() {
    //let start = Instant::now();
    let (n, edges): (usize, ListOfEdges) = read_graph("C:/Users/ldunn/DS210/project/code/src/PA_Roads.txt");
    let graph = Graph::create_undirected(n, &edges);

    /*let graph_creation_time = start.elapsed();
    println!("Time taken to create graph: {:?}", graph_creation_time); 
    takes just under a second using cargo run, about half a second using --release
    
    testing data to see if I have read it correctly using prewritten page rank function 
    from homework - tested the time that it took to run just in case my graph creation wasn't 
    the part slowing down the code

    let start = Instant::now();
    pagerank(&graph);
    let pagerank_time = start.elapsed();
    println!("Time taken for PageRank: {:?}", pagerank_time);
    */
}

