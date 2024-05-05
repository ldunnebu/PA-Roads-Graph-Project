mod graph_reader;
mod distances;
mod bfs_sample;

use graph_reader::{Graph, read_graph};
use crate::bfs_sample::bfs_sample;

fn main() {
    let (n, edges) = read_graph("src/PA_Roads.txt");
    let graph = Graph::create_undirected(n, &edges);
    bfs_sample(&graph, 50000, n);
}


//tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::distances::avg_shortest;

    #[test]
    fn graph_creation() {
        let (n, edges) = read_graph("C:/Users/ldunn/DS210/project/code/src/project_test.txt");
        let graph = Graph::create_undirected(n, &edges);

        assert_eq!(n, 4);  //graph has 4 nodes
        assert_eq!(graph.outedges.len(), 4);
        assert!(graph.outedges[0].contains(&1)); //node 1 is from 0-1
        assert!(graph.outedges[1].contains(&0)); //show undirectedness
    }

    #[test]
    fn test_avg_shortest() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let graph = Graph::create_undirected(4, &edges); //4 nodes 0,1,2,3
        let (avg_distance, _, _, _, _) = avg_shortest(&graph, 0);

        assert_eq!(avg_distance, 2.0);  //average distance from node 0 to others is 1+2+3/3
    }

    #[test]
    fn min_length() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let graph = Graph::create_undirected(4, &edges); //4 nodes 0,1,2,3
        let (_, _, _, min_length, _) = avg_shortest(&graph, 0);

        assert_eq!(min_length, 1);
    }

    #[test]
    fn avg_shortest_no_connect() {
        let edges = vec![(0, 1)]; 
        let graph = Graph::create_undirected(3, &edges); //Nodes 0, 1, 2 - 2 has no edges
        let (avg_distance, _, _, _, _) = avg_shortest(&graph, 2);

        assert_eq!(avg_distance, 0.0);  //therefore, distance to 2 is 0 because it doesn't exist
    }

    #[test]
    fn test_stats() {
        let edges = vec![(0, 1), (1, 2), (2, 3)];
        let graph = Graph::create_undirected(4, &edges); //4 nodes 0,1,2,3
        let (_, _, max_path, _, _) = avg_shortest(&graph, 0);

        assert_eq!(max_path.unwrap(), (0,3));
    }
}