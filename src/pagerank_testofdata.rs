use crate::graph_reader::{Graph};
use rand::Rng;

// page rank - testing the dataset on a function that I know already works to see if I have read it properly. Had to limit the walks and steps to 5 and 5 to make the computation quicker because it was taking more than 10 minutes with no result.
pub fn pagerank(g: &Graph) -> Vec<usize> {
    let mut counts = vec![0_usize; g.n];
    for i in 0..(g.n){ //verticies
        for _ in 0..5 { //walks
            let mut current = i;
            for _ in 0..5 { //steps
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