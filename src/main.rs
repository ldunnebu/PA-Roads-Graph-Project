mod graph_reader;
use graph_reader::{Graph, ListOfEdges, read_graph};
use crate::graph_reader::pagerank;

fn main() {
    let (n, edges): (usize, ListOfEdges) = read_graph("C:/Users/ldunn/DS210/project/code/src/PA_Roads.txt");
    let graph = Graph::create_undirected(n, &edges);
    pagerank(&graph);
}

