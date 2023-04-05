use fmt::Display;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::{BufRead};
use std::fmt;
use rand::prelude::*;


#[derive(Debug)]
pub(crate) struct Hypergraph<V,E> {
    nodes: HashMap<V, HashSet<E>>,
    edges: HashMap<E, HashSet<V>>,
}

impl<V, E> Hypergraph<V, E> {
    pub(crate) fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub(crate) fn add_node(&mut self, node: V, edge: E) where
        V: Hash, V: Eq, E: Hash, E: Eq

    {
        self.nodes.entry(node).or_insert_with(HashSet::new).insert(edge);
    }

    pub(crate) fn add_edge(&mut self, edge: E, nodes: HashSet<V>) where
        V: Hash, V: Eq, E: Hash, E: Eq

    {
        let nodes_of_edge = self.edges
            .entry(edge)
            .or_insert_with(HashSet::new);

        for node in nodes {
            nodes_of_edge.insert(node);
        }

    }
}

impl Hypergraph<String, usize> {
    pub(crate) fn from_reader<R: BufRead>(reader: R) -> Self where {
        let mut hypergraph = Self::new();
        let mut line_num = 0;

        for line in reader.lines() {
            let line = line.unwrap_or("-".to_owned());
            if line == "-" {
                break;
            }

            let mut words = HashSet::new();
            for word in line.split_whitespace() {
                words.insert(word.to_owned());
            }

            if words.len() >= 2 {
                for word in words.borrow() {
                    hypergraph.add_node(word.to_owned(), line_num);
                }
                hypergraph.add_edge(line_num, words);
            }

            line_num += 1;
        }

        hypergraph
    }

    pub(crate) fn random(num_nodes: usize, num_edges: usize) -> Self {
        let mut hypergraph = Self::new();
        let mut rng = thread_rng();

        for edge_num in 0..num_edges {
            let num_nodes_in_edge = rng.gen_range(1..num_nodes);
            let mut nodes_in_edge = HashSet::new();

            for _ in 0..num_nodes_in_edge {
                let node_index = rng.gen_range(0.. num_nodes);
                let node_name = generate_node_name(node_index);
                nodes_in_edge.insert(node_name.to_string());
            }

            for node in &nodes_in_edge {
                hypergraph.add_node(node.clone(), edge_num);
            }

            hypergraph.add_edge(edge_num, nodes_in_edge);
        }

        hypergraph
    }
}

fn generate_node_name(index: usize) -> String {
    let mut result = String::new();
    let mut index = index;

    loop {
        let remainder = index % 26;
        let digit = ('a' as u8 + remainder as u8) as char;
        result.push(digit);
        index /= 26;
        if index == 0 {
            break;
        }
        index -= 1;
    }

    result.chars().rev().collect()
}

impl <V, E> Hypergraph<V, E> where
    V: Eq + Hash + Clone,
    E: Eq + Hash + Clone
{
    pub(crate) fn adjacency_matrix(&self) -> Vec<Vec<bool>> {
        let nodes = self.nodes.keys().cloned().collect::<Vec<V>>();
        let edges = self.edges.keys().cloned().collect::<Vec<E>>();
        let mut matrix = vec![vec![false; edges.len()]; nodes.len()];

        for (node_index, node) in nodes.iter().enumerate() {
            if let Some(edges_set) = self.nodes.get(node) {
                for edge in edges_set {
                    if let Some(edge_index) = edges.iter().position(|e| e == edge) {
                        matrix[node_index][edge_index] = true;
                    }
                }
            }
        }

        matrix
    }
}

impl<V, E> Display for Hypergraph<V, E>
    where
        V: Eq + Hash + Display + Clone,
        E: Eq + Hash + Display + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let matrix = self.adjacency_matrix();

        for matrix_line in matrix {
            writeln!(f)?;
            for boolean_value in matrix_line {
                write!(f, "{}", if boolean_value { "@" } else { " " })?;
            }
        }

        Ok(())
    }
}
