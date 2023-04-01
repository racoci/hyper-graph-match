mod hyper_graph;

use std::io::{self, BufRead};

fn main() {
    println!("\n\nPlease input the first hypergraph, one hyperedge per line,\n\
     with the vertices of each hyperedge separated by whitespace. \n\
     Indicate the end of the hypergraph by typing a single dash (-) on\n\
      a line by itself.\n\n");
    let stdin = io::stdin();
    let hg1 = hyper_graph::Hypergraph::from_reader(stdin.lock());

    println!("Please input the second hypergraph, using the same format as the first hypergraph.");
    let hg2 = hyper_graph::Hypergraph::from_reader(stdin.lock());

    println!("Hypergraph 1: {:#?}", hg1);
    println!("Hypergraph 2: {:#?}", hg2);
}