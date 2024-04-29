use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

type Vertex = usize;
pub type ListOfEdges = Vec<(Vertex, Vertex)>;
pub type AdjacencyLists = Vec<Vec<Vertex>>;

#[derive(Debug)]
pub struct Graph {
    pub n: usize, // vertex labels in {0,...,n-1}
    pub outedges: AdjacencyLists,
}

impl Graph {
    pub fn add_directed_edges(&mut self, edges: &ListOfEdges) {
        for (u,v) in edges {
            self.outedges[*u].push(*v);
        }
    }

    pub fn sort_graph_lists(&mut self) {
        for l in self.outedges.iter_mut() {
            l.sort();
        }
    }

    pub fn reverse_edges(list:&ListOfEdges) -> ListOfEdges {
        let mut new_list = vec![];
        for (u,v) in list {
            new_list.push((*v,*u));
        }
        new_list
    }

    pub fn create_undirected(n: usize, edges: &ListOfEdges) -> Graph {
        let mut g = Self::create_directed(n, edges);
        g.add_directed_edges(&Graph::reverse_edges(edges));
        g.sort_graph_lists();
        g
    }

    pub fn create_directed(n:usize,edges: &ListOfEdges) -> Graph {
        let mut g = Graph{n,outedges:vec![vec![];n]};
        g.add_directed_edges(edges);
        g.sort_graph_lists();
        g                                        
    }

}

//read graph
pub fn read_graph(path: &str) -> (usize, ListOfEdges) {
    let mut result: ListOfEdges = Vec::new(); //return variable with tuples of pairs of signed integers
    let file = File::open(path).expect("Could not open file"); //opens file
    let buf_reader = std::io::BufReader::new(file).lines(); //creates a reader for the file that allows line by line reading, command prompt recommended mutable
    let mut n: usize = 0; //first entry
    for (i, line) in buf_reader.enumerate() {
        let line_str = line.expect("Error reading");
        if i == 0 {
            n = line_str.trim().parse::<usize>().unwrap();
        }
        else{
            let v: Vec<&str> = line_str.trim().split(' ').collect();
            let vertex = v[0].parse::<usize>().unwrap();
            let edge = v[1].parse::<usize>().unwrap();
            result.push((vertex, edge)); //pushes integers as a tuple into result
        }  
    } 
    return (n, result);
}

//page rank
pub fn pagerank(g: &Graph) -> Vec<usize> {
    let mut counts = vec![0_usize; g.n];
    for i in 0..(g.n){ //verticies
        for _ in 0..100 { //100 walks
            let mut current = i;
            for _ in 0..100 { //100 steps
                let random_select = rand::thread_rng().gen_range(0..10);
                if g.outedges[current].len() == 0 || random_select == 1 {
                    current = rand::thread_rng().gen_range(0..(g.n)); //random vertex
                }
                else {
                    let new_index = rand::thread_rng().gen_range(0..g.outedges[current].len()); //neighboring vertex
                    current = g.outedges[current][new_index];
                }
            }
            counts[current] += 1; //number of terminations at each vertex
        }
    }   
    let mut counts_i: Vec<(usize, &usize)> = counts.iter().enumerate().collect();
    counts_i.sort_by_key(|&(_, v)| std::cmp::Reverse(v));
    let length = std::cmp::min(5, counts.len());
    for (index, value) in counts_i.iter().take(length) {
        let estimated_rank: f32 = (*(*value) as f32) / (100.0 * (g.n as f32));
        println!("vertex {}: approximate PageRank {}", index, estimated_rank)
    }
    return counts;
}