use fmt::Display;
use std::borrow::Borrow;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::io::{BufRead};
use std::fmt;
use rand::prelude::*;


#[derive(Debug, Clone)]
pub(crate) struct Hypergraph<V: Eq + Hash, E: Eq + Hash> {
    pub(crate) nodes: HashMap<V, HashSet<E>>,
    pub(crate) edges: HashMap<E, HashSet<V>>,
}

impl<V: PartialEq, E: PartialEq> PartialEq for Hypergraph<V, E> where
    V: Eq + Hash,
    E: Eq + Hash
{
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

trait Permutable<K,V> {
    fn permute(&self, permutation: &[usize]) -> Self;
}

// This trait defines a method to permute the keys of a HashMap
impl<K: Eq + Hash + Clone, V: Clone> Permutable<K, V> for HashMap<K, V> {
    // This method takes a reference to a HashMap and a slice of usize as a permutation
    // It returns a new HashMap with the same values but with permuted keys
    fn permute(&self, permutation: &[usize]) -> Self {
        // If the HashMap is empty, return an empty HashMap
        if self.is_empty() {
            return HashMap::new()
        }

        // If the permutation is empty, return a clone of the original HashMap
        if permutation.is_empty() {
            return self.clone()
        }

        // Check if the permutation is valid, i.e. it contains indices within the range of the HashMap keys
        for index in permutation {
            if *index > permutation.len() {
                println!("Invalid permutation {:#?}", permutation);
                return self.clone()
            }
        }

        // Check if the permutation is smaller than the number of keys in the HashMap
        // If so, return an empty HashMap
        if permutation.len() < self.keys().len() {
            println!("Permutation is smaller than set of keys");
            return return HashMap::new()
        }

        // Check if the permutation is larger than the number of keys in the HashMap
        // If so, return a clone of the original HashMap
        if permutation.len() > self.keys().len() {
            println!("Permutation {} is grater than set of keys {}",
                     permutation.len(),
                     self.keys().len()
            );
            return self.clone()
        }

        // Create a new HashMap with the same capacity as the original one
        let mut new_map = HashMap::<K,V>::with_capacity(self.len());
        // Collect the keys of the original HashMap into a vector
        let keys: Vec<K> = self.keys().cloned().collect();
        // Iterate over the keys and their indices
        for (index, key) in keys.iter().enumerate() {
            // Get a reference to the value associated with the current key
            let old_value_reference = self.get(key).unwrap();
            // Get the permuted index from the permutation slice
            let permuted_index = permutation.get(index).unwrap();
            // Get the permuted key from the keys vector using the permuted index
            let permuted_key = keys.get(*permuted_index).unwrap().clone();
            // Clone the old value
            let old_value = old_value_reference.clone();
            // Insert the permuted key and the old value into the new HashMap
            let x = new_map.insert(permuted_key, old_value);
        }
        // Return the new HashMap with permuted keys
        new_map
    }
}


impl<V: Eq + Hash + Clone, E: Eq + Hash + Clone> Hypergraph<V, E> {
    pub(crate) fn permute(
        &self,
        node_permutation: &[usize],
        edge_permutation: &[usize],
    ) -> Self {
        let nodes = self.nodes.permute(node_permutation);
        let edges = self.edges.permute(edge_permutation);
        Hypergraph { nodes, edges }
    }
}

// This struct represents a hypergraph with vertices of type V and edges of type E
impl<V: Eq + Hash, E: Eq + Hash> Hypergraph<V, E> {
    // This method creates a new empty hypergraph
    pub(crate) fn new() -> Self {
        Self {
            // Initialize an empty HashMap to store the nodes and their adjacent edges
            nodes: HashMap::new(),
            // Initialize an empty HashMap to store the edges and their incident nodes
            edges: HashMap::new(),
        }
    }

    // This method adds a node and an edge to the hypergraph
    // It also updates the adjacency relation between the node and the edge
    pub(crate) fn add_node(&mut self, node: V, edge: E) where
        V: Hash + Eq,
        E: Hash + Eq
    {
        // Insert the node into the nodes HashMap if it does not exist
        // Get a mutable reference to the HashSet of adjacent edges for the node
        // Insert the edge into the HashSet
        self.nodes.entry(node).or_insert_with(HashSet::new).insert(edge);
    }

    // This method adds an edge and a set of nodes to the hypergraph
    // It also updates the incidence relation between the edge and the nodes
    pub(crate) fn add_edge(&mut self, edge: E, nodes: HashSet<V>) where
        V: Hash + Eq,
        E: Hash + Eq
    {
        // Insert the edge into the edges HashMap if it does not exist
        // Get a mutable reference to the HashSet of incident nodes for the edge
        let nodes_of_edge = self.edges
            .entry(edge)
            .or_insert_with(HashSet::new);

        // Iterate over the nodes in the input set
        for node in nodes {
            // Insert each node into the HashSet of incident nodes for the edge
            nodes_of_edge.insert(node);
        }

    }
}


// This struct represents a hypergraph with vertices of type String and edges of type usize
impl Hypergraph<String, usize> {
    // This method creates a hypergraph from a reader that provides lines of text
    // Each line represents an edge and each word in the line represents a node
    // The edge is assigned a number based on the line number
    pub(crate) fn from_reader<R: BufRead>(reader: R) -> Self where {
        // Create a new empty hypergraph
        let mut hypergraph = Self::new();
        // Initialize a variable to keep track of the line number
        let mut line_num = 0;

        // Iterate over the lines from the reader
        for line in reader.lines() {
            // Unwrap the line or use "-" as a default value
            let line = line.unwrap_or("-".to_owned());
            // If the line is "-", stop reading
            if line == "-" {
                break;
            }

            // Create a HashSet to store the words in the line
            let mut words = HashSet::new();
            // Split the line by whitespace and insert each word into the HashSet
            for word in line.split_whitespace() {
                words.insert(word.to_owned());
            }

            // If the HashSet has at least two words, add them as nodes and an edge to the hypergraph
            if words.len() >= 2 {
                // For each word in the HashSet, add it as a node with the line number as the edge
                for word in words.borrow() {
                    hypergraph.add_node(word.to_owned(), line_num);
                }
                // Add the line number as an edge with the HashSet of words as the nodes
                hypergraph.add_edge(line_num, words);
            }

            // Increment the line number by one
            line_num += 1;
        }

        // Return the hypergraph
        hypergraph
    }

    // This method creates a random hypergraph with a given number of nodes and edges
    pub(crate) fn random(num_nodes: usize, num_edges: usize) -> Self {
        // Create a new empty hypergraph
        let mut hypergraph = Self::new();
        // Create a random number generator
        let mut rng = thread_rng();

        // For each edge number from 0 to num_edges, generate a random edge with random nodes
        for edge_num in 0..num_edges {
            // Generate a random number of nodes in the edge, at least two and at most num_nodes
            let num_nodes_in_edge = rng.gen_range(2..=max(2,num_nodes));
            // Create a HashSet to store the nodes in the edge
            let mut nodes_in_edge = HashSet::new();

            // For each node in the edge, generate a random node index and name
            for _ in 0..num_nodes_in_edge {
                let node_index = rng.gen_range(0.. num_nodes);
                let node_name = generate_node_name(node_index);
                // Insert the node name into the HashSet
                nodes_in_edge.insert(node_name.to_string());
            }

            // For each node in the HashSet, add it as a node with the edge number as the edge
            for node in &nodes_in_edge {
                hypergraph.add_node(node.clone(), edge_num);
            }

            // Add the edge number as an edge with the HashSet of nodes as the nodes
            hypergraph.add_edge(edge_num, nodes_in_edge);
        }

        // Return the hypergraph
        hypergraph
    }
}

/// This function generates a node name from a given index
/// The node name is a string of lowercase letters, starting from "a" for index 0
/// The node name follows the pattern of a base-26 number system, where "z" is followed by "ba"
/// For example, index 0 -> "a", index 25 -> "z", index 26 -> "ba", index 51 -> "bz", etc.
fn generate_node_name(index: usize) -> String {
    // Create an empty string to store the result
    let mut result = String::new();
    // Create a mutable copy of the index
    let mut index = index;

    // Loop until the index becomes zero
    loop {
        // Get the remainder of the index divided by 26
        let remainder = index % 26;
        // Convert the remainder to a lowercase letter
        let digit = ('a' as u8 + remainder as u8) as char;
        // Push the letter to the result string
        result.push(digit);
        // Divide the index by 26
        index /= 26;
        // If the index is zero, break the loop
        if index == 0 {
            break;
        }
        // Subtract one from the index to account for the zero-based numbering
        index -= 1;
    }

    // Reverse the result string and return it
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

// This trait defines a method to display a hypergraph as a matrix of symbols
impl<V, E> Display for Hypergraph<V, E>
    where
    // The vertices and edges of the hypergraph must implement the Display, Eq, Hash and Clone traits
        V: Eq + Hash + Display + Clone,
        E: Eq + Hash + Display + Clone,
{
    // This method takes a reference to a hypergraph and a mutable reference to a formatter
    // It writes the hypergraph as a matrix of symbols to the formatter
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Get the adjacency matrix of the hypergraph as a vector of vectors of booleans
        let matrix = self.adjacency_matrix();

        // Use the precision field of the formatter to get the symbols to use for true and false values in the matrix
        // The precision field is an optional usize value that can be passed with a format like "{:.2}"
        // We can use it to encode two ASCII characters as one usize value
        // For example, "{:.8481}" would encode '@' and ' ' as 8481 = 64 * 128 + 32
        // We can also use the Default trait to get the default values for the symbols if they are not given
        let (true_symbol, false_symbol) = match f.precision() {
            Some(p) => {
                // Decode the precision value into two ASCII characters
                let true_symbol = (p / 128) as u8 as char;
                let false_symbol = (p % 128) as u8 as char;
                (true_symbol, false_symbol)
            }
            None => {
                // Use the default values for the symbols
                ('@', ' ')
            }
        };

        // Iterate over the rows of the matrix
        for matrix_line in matrix {
            // Write a new line to the formatter
            writeln!(f)?;
            // Iterate over the columns of the matrix
            for boolean_value in matrix_line {
                // Write the true symbol if the value is true or the false symbol if the value is false to the formatter
                write!(f, "{}", if boolean_value { true_symbol } else { false_symbol })?;
            }
        }

        // Return Ok if no errors occurred
        Ok(())
    }
}