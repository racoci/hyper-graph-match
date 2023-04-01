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

fn read_hypergraph<R: BufRead>(reader: R) -> Hypergraph {
    let mut hypergraph = Hypergraph::new();
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

fn main() {
    println!("\n\nPlease input the first hypergraph, one hyperedge per line,\n\
     with the vertices of each hyperedge separated by whitespace. \n\
     Indicate the end of the hypergraph by typing a single dash (-) on\n\
      a line by itself.\n\n");
    let stdin = io::stdin();
    let hg1 = read_hypergraph(stdin.lock());

    println!("Please input the second hypergraph, using the same format as the first hypergraph.");
    let hg2 = read_hypergraph(stdin.lock());

    println!("Hypergraph 1: {:#?}", hg1);
    println!("Hypergraph 2: {:#?}", hg2);
}