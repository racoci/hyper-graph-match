mod hyper_graph;

use std::io::{self, BufRead};
use hyper_graph::Hypergraph;

fn main() {
    println!("\n\nPlease input the first hypergraph, one hyperedge per line,\n\
     with the vertices of each hyperedge separated by whitespace. \n\
     Indicate the end of the hypergraph by typing a single dash (-) on\n\
      a line by itself.\n\n");
    let stdin = io::stdin();
    let hg1 = Hypergraph::<String, usize>::from_reader(stdin.lock());

    println!("Please input the second hypergraph, using the same format as the first hypergraph.");
    let hg2 = Hypergraph::<String, usize>::from_reader(stdin.lock());

    println!("Please enter the number of nodes for the third hypergraph:");
    let num_nodes_hg3: usize = read_usize(stdin.lock());

    println!("Please enter the number of hyperedges for the third hypergraph:");
    let num_edges_hg3: usize = read_usize(stdin.lock());

    let hg3 = Hypergraph::<String, usize>::random(
        num_nodes_hg3,
        num_edges_hg3
    );

    println!("Hypergraph 1: {}", hg1);
    println!("Hypergraph 2: {:#?}", hg2);
    println!("Hypergraph 3: {:#?}", hg3);
}

fn read_usize<R: BufRead>(mut reader: R) -> usize {
    let mut input = String::new();
    reader.read_line(&mut input).expect("Failed to read input.");
    return input.trim().parse().unwrap();
}