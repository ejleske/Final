use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use crate::Graph;
type Vertex = usize; // Define Vertex as usize

// Function to find unique nodes in a list of edges
pub fn unique_nodes(edges: &Vec<(usize, usize)>) -> HashSet<usize> {
    let mut unique_nodes_set: HashSet<usize> = HashSet::new();
    for (i, j) in edges.iter() {
        unique_nodes_set.insert(*i);
        unique_nodes_set.insert(*j);
    }
    unique_nodes_set
}

// Function to create an adjacency list representation of the graph
pub fn adjacency_list(edges: Vec<(usize, usize)>, unique_nodes: HashSet<usize>) -> Vec<Vec<usize>> {
    let num_unique = unique_nodes.len();
    let vec_unique_nodes: Vec<&usize> = unique_nodes.iter().collect();
    
    let mut graph_list: Vec<Vec<usize>> = vec![vec![]; num_unique];
    let mut node_map: HashMap<usize, usize> = HashMap::new();

    for (idx, val) in vec_unique_nodes.iter().enumerate() {
        node_map.insert(**val, idx);
    }

    // Fill the adjacency list based on the edges provided
    for (start, destination) in edges {
        graph_list[node_map[&start]].push(node_map[&destination]);
        graph_list[node_map[&destination]].push(node_map[&start]); // This is for undirected graph
    }

    // Return the constructed adjacency list
    graph_list
}

// Helper function to calculate distances from the start vertex using BFS
pub fn distances_bfs(start: Vertex, graph: &Graph, distances: &mut Vec<Option<u32>>) {
    distances[start] = Some(0);
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(v) = queue.pop_front() {
        for &u in &graph.outedges[v] {
            if distances[u].is_none() {
                distances[u] = Some(distances[v].unwrap() + 1);
                queue.push_back(u);
            }
        }
    }
}