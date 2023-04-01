use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::{BufRead};



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
}