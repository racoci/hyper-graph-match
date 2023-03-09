use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Hypergraph {
    vertices: HashMap<String, HashSet<usize>>,
    edges: Vec<HashSet<String>>,
}

impl Hypergraph {
    fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: Vec::new(),
        }
    }

    fn add_vertex(&mut self, name: String, line_num: usize) {
        self.vertices
            .entry(name)
            .or_insert_with(HashSet::new)
            .insert(line_num);
    }

    fn add_edge(&mut self, vertices: HashSet<String>) {
        self.edges.push(vertices);
    }
}

fn main() {
    let mut hypergraph = Hypergraph::new();
    let mut line_num = 0;
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == "-" {
            break;
        }
        let mut vertices = HashSet::new();
        for word in line.split_whitespace() {
            hypergraph.add_vertex(word.to_owned(), line_num);
            vertices.insert(word.to_owned());
        }
        hypergraph.add_edge(vertices);
        line_num += 1;
    }

    println!("{:#?}", hypergraph);
}
