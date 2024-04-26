use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader};
use csv::ReaderBuilder;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

// Define a struct to represent the graph
struct Graph {
    edges: HashMap<u32, Vec<u32>>,
}

impl Graph {
    // Constructor function to create a new empty graph
    fn new() -> Self {
        Graph {
            edges: HashMap::new(),
        }
    }

    // Function to add an edge to the graph
    fn add_edge(&mut self, from_node: u32, to_node: u32) {
        // Add 'to_node' to the list of neighbors of 'from_node'
        self.edges
            .entry(from_node)
            .or_insert_with(Vec::new)
            .push(to_node);
    }

    // Function to find the shortest path lengths from a source node to all other nodes
    fn shortest_path_lengths(&self, source: u32) -> HashMap<u32, u32> {
        let mut distances: HashMap<u32, u32> = HashMap::new();
        let mut visited: HashSet<u32> = HashSet::new();
        let mut queue: Vec<(u32, u32)> = vec![(source, 0)];

        while let Some((node, dist)) = queue.pop() {
            if !visited.contains(&node) {
                visited.insert(node);
                distances.insert(node, dist);

                if let Some(neighbors) = self.edges.get(&node) {
                    for &neighbor in neighbors {
                        if !visited.contains(&neighbor) {
                            queue.push((neighbor, dist + 1));
                        }
                    }
                }
            }
        }

        distances
    }

    // Function to print the average shortest path length and the most used nodes in these shortest paths
    fn analyze_shortest_paths(&self) {
        let mut total_shortest_paths = 0;
        let mut total_shortest_path_length = 0;
        let mut node_frequency: HashMap<u32, u32> = HashMap::new();
    
        // Iterate over each node as the source node
        for &source_node in self.edges.keys() {
            let shortest_path_lengths = self.shortest_path_lengths(source_node);
            total_shortest_paths += shortest_path_lengths.len();
    
            for (&node, &dist) in &shortest_path_lengths {
                total_shortest_path_length += dist as usize;
                *node_frequency.entry(node).or_insert(0) += 1;
            }
        }
    
        // Debug prints
        println!("Total Shortest Paths: {}", total_shortest_paths);
        println!("Total Shortest Path Length: {}", total_shortest_path_length);
        println!("Node Frequency: {:?}", node_frequency);
    
        // Calculate the average shortest path length
        let average_shortest_path_length = if total_shortest_paths > 0 {
            total_shortest_path_length as f64 / total_shortest_paths as f64
        } else {
            0.0
        };
    
        // Find the most used nodes in shortest paths
        let mut most_used_nodes: Vec<(u32, u32)> = node_frequency.into_iter().collect();
        most_used_nodes.sort_by(|(_, count1), (_, count2)| count2.cmp(count1));
    
        println!("Average Shortest Path Length: {:.2}", average_shortest_path_length);
        println!("Most Used Nodes in Shortest Paths:");
    
        for (node, count) in most_used_nodes.iter().take(5) {
            println!("Node {}: {} times", node, count);
        }
    }
}

// Function to read the dataset and construct the graph
fn read_dataset(filename: &str) -> Result<Graph, Box<dyn Error>> {
    let mut graph = Graph::new();

    // Open the CSV file
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(reader);

    // Iterate over each record in the CSV file
    for result in rdr.records() {
        let record = result?;
        if let Some(from_node_str) = record.get(0) {
            if let Some(to_node_str) = record.get(1) {
                let from_node: u32 = from_node_str.parse()?;
                let to_node: u32 = to_node_str.parse()?;
                // Add the edge to the graph
                graph.add_edge(from_node, to_node);
            }
        }
    }

    Ok(graph)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the dataset and construct the graph
    let graph = read_dataset("C:\\Users\\ldunn\\DS210\\FinalProject\\code\\src\\PA_Roads.csv")?;

    // Analyze shortest paths and print results
    graph.analyze_shortest_paths();

    Ok(())
}