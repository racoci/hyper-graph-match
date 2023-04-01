use std::collections::{HashMap, HashSet};
use std::io::{BufRead};



#[derive(Debug)]
pub(crate) struct Hypergraph {
    vertices: HashMap<String, HashSet<usize>>,
    edges: Vec<HashSet<String>>,
}

impl Hypergraph {
    pub(crate) fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub(crate) fn from_reader<R: BufRead>(reader: R) -> Self {
        let mut hypergraph = Self::new();
        let mut line_num = 0;

        for line in reader.lines() {
            let line = line.unwrap();
            if line == "-" {
                break;
            }
            let mut vertices = HashSet::new();
            for word in line.split_whitespace() {
                hypergraph.add_vertex(word.to_owned(), line_num);
                vertices.insert(word.to_owned());
            }
            if vertices.len() >= 2 {
                hypergraph.add_edge(vertices);
                line_num += 1;
            }
        }

        hypergraph
    }

    pub(crate) fn add_vertex(&mut self, name: String, line_num: usize) {
        self.vertices
            .entry(name)
            .or_insert_with(HashSet::new)
            .insert(line_num);
    }

    pub(crate) fn add_edge(&mut self, vertices: HashSet<String>) {
        self.edges.push(vertices);
    }
}