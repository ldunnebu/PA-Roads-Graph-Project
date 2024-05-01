use std::fs::File;
use std::io::{self, BufRead};

pub type Vertex = usize;
pub type ListOfEdges = Vec<(Vertex, Vertex)>;
pub type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
pub struct Graph {
    pub n: usize, // vertex labels in {0,...,n-1}
    pub outedges: AdjacencyLists,
}

impl Graph {
    pub fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (u, v) in edges {
            if self.outedges.len() <= *u {
                self.outedges.resize(*u + 1, vec![]);
            }
            self.outedges[*u].push(*v);
        }
    }

    pub fn sort_graph_lists(&mut self) {
        self.outedges.iter_mut().for_each(|list| {
            list.sort();
        });
    }
    
    /* parallel running to try and speed it up because it is taking really long to run
    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }
    */

    pub fn reverse_edges(list: &ListOfEdges) -> ListOfEdges {
        list.iter().map(|(u, v)| (*v, *u)).collect()
    }

    pub fn create_undirected(n: usize, edges: &ListOfEdges) -> Self {
        let mut g = Self::create_directed(n, edges);
        let reversed_edges = Self::reverse_edges(edges);
        g.add_directed_edges(&reversed_edges);
        g.sort_graph_lists();
        g
    }

    pub fn create_directed(n: usize, edges: &ListOfEdges) -> Self {
        let mut g = Graph { n, outedges: vec![vec![]; n] };
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g
    }
}

pub fn read_graph(path: &str) -> (usize, ListOfEdges) {
    let file = File::open(path).expect("Could not open file");
    let buf_reader = io::BufReader::new(file).lines();
    let mut n = 0;
    let mut edges = Vec::new();

    for (i, line) in buf_reader.enumerate() {
        let line = line.expect("Error reading line");
        if i == 0 {
            n = line.trim().parse().unwrap();
        } else {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let u = parts[0].parse::<usize>().unwrap();
            let v = parts[1].parse::<usize>().unwrap();
            edges.push((u, v));
        }
    }
    (n, edges)
}