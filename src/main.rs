mod hyper_graph;

use std::io::{self, BufRead};
use rand::{Rng};
use rand::seq::SliceRandom;
use hyper_graph::Hypergraph;

fn random_permutation(n: usize) -> Vec<usize> {
    let mut permutation: Vec<usize> = (0..n).collect();
    let mut rng = rand::thread_rng();
    permutation.shuffle(&mut rng);
    permutation
}

fn test_canonization(max_nodes: usize, num_tests: usize, num_graphs: usize) {
    let mut rng = rand::thread_rng();

    for i in 0..num_graphs {
        let num_nodes = rng.gen_range(1..=max_nodes);
        let num_edges = rng.gen_range(1..=(num_nodes));
        let mut hg = Hypergraph::random(num_nodes, num_edges);

        println!("Testing {} permutations for the hypergraph with {} nodes and {} edges: \n {}",
                 num_tests, num_nodes, num_edges, hg);

        for j in 0..num_tests {
            let node_permutation = random_permutation(hg.nodes.len());
            let edge_permutation = random_permutation(hg.edges.len());

            let permuted_hg = hg.permute(&node_permutation, &edge_permutation);

            if permuted_hg != hg {
                println!(
                    "Test {} failed for graph {} with {} nodes and {} edges: {}",
                    j + 1,
                    i + 1,
                    num_nodes,
                    num_edges,
                    permuted_hg
                );
                return;
            }
        }
    }

    println!("All tests passed!");
}

fn read_usize<R: BufRead>(mut reader: R) -> usize {
    let mut input = String::new();
    reader.read_line(&mut input).expect("Failed to read input.");
    return input.trim().parse().unwrap();
}

fn main() {
    let stdin = io::stdin();

    // Read the maximum number of nodes to test
    println!("Enter the maximum number of nodes to test: ");
    let max_nodes = read_usize(stdin.lock());

    // Read the number of tests per graph
    println!("Enter the number of tests per graph: ");
    let num_tests = read_usize(stdin.lock());

    // Read the number of graphs to test
    println!("Enter the number of graphs to test: ");
    let num_graphs = read_usize(stdin.lock());

    // Call the test_canonization function with the specified parameters
    test_canonization(max_nodes, num_tests, num_graphs);
}