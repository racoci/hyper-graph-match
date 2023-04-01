use std::collections::{HashMap, HashSet};


type Vertex = usize;
type VertexSet = HashSet<Vertex>;

#[derive(Debug)]
struct BipartiteGraph {
    left_vertices: VertexSet,
    right_vertices: VertexSet,
    edges: HashMap<Vertex, VertexSet>,
}

impl BipartiteGraph {
    fn new(left_vertices: VertexSet, right_vertices: VertexSet, edges: HashMap<Vertex, VertexSet>) -> Self {
        BipartiteGraph { left_vertices, right_vertices, edges }
    }
}

#[derive(Debug)]
struct VertexMapping(HashMap<Vertex, Vertex>);

impl VertexMapping {
    fn new() -> Self {
        VertexMapping(HashMap::new())
    }

    fn insert(&mut self, u: Vertex, v: Vertex) {
        self.0.insert(u, v);
    }
}

fn hopcroft_karp(graph: &BipartiteGraph) -> VertexMapping {
    // Initialize empty matching and distance
    let mut matching = VertexMapping::new();
    let mut dist = HashMap::new();

    // Initialize queue with all free left vertices
    let mut queue = Vec::new();
    for &u in &graph.left_vertices {
        if !matching.0.contains_key(&u) {
            dist.insert(u, 0);
            queue.push(u);
        }
    }

    // Run BFS to find shortest augmenting paths
    while !queue.is_empty() {
        let u = queue.pop().unwrap();

        // Stop when we have found a maximal matching
        if !graph.edges.contains_key(&u) {
            continue;
        }

        for &v in &graph.edges[&u] {
            // If we have not seen this vertex before or if we can extend
            // the augmenting path by making the matching of v a little bigger
            if !dist.contains_key(&v) || dist[&v] == dist[&u] + 1 {
                dist.insert(v, dist[&u] + 1);
                if let Some(&w) = matching.0.get(&v) {
                    queue.push(w);
                    dist.insert(w, dist[&v] + 1);
                } else {
                    // Found an augmenting path, update the matching
                    let mut u = u;
                    let mut v = v;
                    while let Some(&w) = matching.0.get(&u) {
                        matching.insert(u, v);
                        matching.insert(v, u);
                        u = w;
                        v = matching.0[&w];
                    }
                    matching.insert(u, v);
                    return matching;
                }
            }
        }
    }

    // If no augmenting path was found, remove this vertex
    // from the set of free vertices and try again
    VertexMapping::new()
}

fn is_contained(hypergraph1: &Hypergraph, hypergraph2: &Hypergraph) -> Option<HashMap<String, String>> {
    // Check if the number of vertices in hypergraph2 is smaller than in hypergraph1
    if hypergraph2.vertices.len() > hypergraph1.vertices.len() {
        return None;
    }

    // Create an empty mapping from vertices in hypergraph1 to vertices in hypergraph2
    let mut map: HashMap<String, String> = HashMap::new();

    // For each vertex in hypergraph2, find a matching vertex in hypergraph1
    for vertex2 in &hypergraph2.vertices {
        let mut matching_vertices = Vec::new();
        for vertex1 in &hypergraph1.vertices {
            if hypergraph1.edges.get(vertex1).unwrap().contains(vertex2) {
                matching_vertices.push(vertex1);
            }
        }

        // If there are no matching vertices, then hypergraph2 is not contained in hypergraph1
        if matching_vertices.is_empty() {
            println!("No matching vertices found for vertex {}", vertex2);
            return None;
        }

        // If there is only one matching vertex, add it to the mapping
        if matching_vertices.len() == 1 {
            let vertex1 = matching_vertices[0];
            map.insert(vertex1.clone(), vertex2.clone());
            println!("Added vertex {} to mapping", vertex1);
        } else {
            // If there are multiple matching vertices, try to recursively check if hypergraph2 is contained
            // in each possible subset of hypergraph1 that contains one of the matching vertices
            let mut contained = false;
            let mut subset_map: Option<HashMap<String, String>> = None;

            for vertex1 in matching_vertices {
                let mut subset_graph = Hypergraph {
                    vertices: hypergraph1.vertices.clone(),
                    edges: HashMap::new(),
                };
                for (v, e) in &hypergraph1.edges {
                    if v == vertex1 {
                        let mut new_e = e.clone();
                        new_e.insert(vertex2.clone());
                        subset_graph.edges.insert(v.clone(), new_e);
                    } else {
                        subset_graph.edges.insert(v.clone(), e.clone());
                    }
                }

                println!("Checking if hypergraph2 is contained in subset of hypergraph1 with vertex {}", vertex1);

                subset_map = is_contained(&subset_graph, hypergraph2);
                if let Some(map2) = subset_map {
                    // Combine the mapping for the subset with the mapping for the rest of hypergraph1
                    let mut new_map = map.clone();
                    for (k, v) in map2 {
                        new_map.insert(k, v);
                    }
                    map = new_map;
                    contained = true;
                    println!("Hypergraph2 is contained in subset of hypergraph1 with vertex {}", vertex1);
                    break;
                }
            }

            // If hypergraph2 is not contained in any subset of hypergraph1, then it is not contained in hypergraph1
            if !contained {
                println!("Hypergraph2 is not contained in any subset of hypergraph1");
                return None;
            }
        }
    }

    // Return the final mapping
    Some(map)
}