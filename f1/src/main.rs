use std::collections::{HashSet, VecDeque, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;
type Vertex = usize; // Define Vertex as usize
type Component = usize; // Define Component type as usize

#[derive(Debug)]
struct Graph {
    n: usize, // Number of vertices
    outedges: Vec<Vec<usize>>, // Adjacency list representation of the graph
}
fn main() {
    let path = "facebook_combined.txt"; // Change this path if necessary
    let (edges, n) = read_file(path); // Read the file and get edges and number of vertices

    // Find the unique nodes
    let unique_nodes_set = unique_nodes(&edges);

    // Create the adjacency list representation of the graph using the provided function
    let adj_list = adjacency_list(edges, unique_nodes_set);

    // Create a graph with the adjacency list
    let graph = Graph {
        n,
        outedges: adj_list,
    };

    // Run BFS from the starting vertex (you can change this to any vertex you want)
    let start_vertex = 0;

    // // Print the adjacency list (adjacency map)
    // println!("Adjacency map:");
    // for (vertex, neighbors) in graph.outedges.iter().enumerate() {
    //     print!("Vertex {}: ", vertex);
    //     for &neighbor in neighbors {
    //         print!("{} ", neighbor);
    //     }
    //     println!();
    // }

    // Calculate mean, max, and median distances
    let (mean_dist, max_dist, median_dist) = calculate_graph_statistics(&graph, start_vertex);

    // Print the results
    println!("Mean distance between different vertices: {:.2}", mean_dist);
    println!("Maximum distance between different vertices: {}", max_dist);
    println!("Median distance between different vertices: {:.2}", median_dist);

}


// Function to read the file and obtain edges and the number of vertices
fn read_file(path: &str) -> (Vec<(usize, usize)>, usize) {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = BufReader::new(file);

    let mut max_vertex = 0;

    // Read each line in the file
    for line in buf_reader.lines() {
        let line_str = line.expect("Error reading line");
        let v: Vec<&str> = line_str.trim().split_whitespace().collect();
        let x = v[0].parse::<usize>().expect("Failed to parse vertex");
        let y = v[1].parse::<usize>().expect("Failed to parse vertex");
        result.push((x, y));

        // Update the maximum vertex index encountered
        max_vertex = max_vertex.max(x).max(y);
    }

    // Calculate the total number of vertices
    let n = max_vertex + 1;

    (result, n)
}

// Function to find unique nodes in a list of edges
fn unique_nodes(edges: &Vec<(usize, usize)>) -> HashSet<usize> {
    let mut unique_nodes_set: HashSet<usize> = HashSet::new();
    for (i, j) in edges.iter() {
        unique_nodes_set.insert(*i);
        unique_nodes_set.insert(*j);
    }
    unique_nodes_set
}

// Function to create an adjacency list representation of the graph
fn adjacency_list(edges: Vec<(usize, usize)>, unique_nodes: HashSet<usize>) -> Vec<Vec<usize>> {
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

// Function to compute and print distance from the start vertex using BFS
fn compute_and_print_distance_bfs(start: Vertex, graph: &Graph) {
    let mut distance: Vec<Option<u32>> = vec![None; graph.n];
    distance[start] = Some(0); // Distance from start to itself is 0

    let mut queue: VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);

    while let Some(v) = queue.pop_front() {
        for &u in &graph.outedges[v] {
            if distance[u].is_none() {
                distance[u] = Some(distance[v].unwrap() + 1);
                queue.push_back(u);
            }
        }
    }
}

// // Function to mark components in the graph using BFS
// fn mark_component_bfs(vertex: Vertex, graph: &Graph, component: &mut Vec<Option<Component>>, component_no: Component) {
//     component[vertex] = Some(component_no);

//     let mut queue = VecDeque::new();
//     queue.push_back(vertex);

//     while let Some(v) = queue.pop_front() {
//         for &w in &graph.outedges[v] {
//             if component[w].is_none() {
//                 component[w] = Some(component_no);
//                 queue.push_back(w);
//             }
//         }
//     }
// }

// Function to calculate graph statistics (mean, max, median distances) from a starting vertex
fn calculate_graph_statistics(graph: &Graph, start_vertex: Vertex) -> (f64, u32, f64) {
    let mut distances = vec![None; graph.n];
    compute_distances(start_vertex, graph, &mut distances);

    // Filter out None values (unreachable vertices)
    let filtered_distances: Vec<u32> = distances.iter().filter_map(|&dist| dist).collect();

    // Calculate mean distance
    let mean_distance = mean(&filtered_distances);

    // Calculate maximum distance
    let max_distance = max(&filtered_distances);

    // Calculate median distance
    let median_distance = median(&filtered_distances);

    (mean_distance, max_distance, median_distance)
}

// Helper function to calculate distances from the start vertex using BFS
fn compute_distances(start: Vertex, graph: &Graph, distances: &mut Vec<Option<u32>>) {
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

// Function to calculate the mean distance from a list of distances
fn mean(distances: &Vec<u32>) -> f64 {
    let total: u32 = distances.iter().copied().sum();
    let count = distances.len();
    total as f64 / count as f64
}

// Function to calculate the maximum distance from a list of distances
fn max(distances: &Vec<u32>) -> u32 {
    *distances.iter().max().unwrap()
}

// Function to calculate the median distance from a list of distances
fn median(distances: &Vec<u32>) -> f64 {
    let mut sorted_distances = distances.clone();
    sorted_distances.sort();

    let len = sorted_distances.len();

    if len % 2 == 1 {
        // If the list length is odd, return the middle element
        sorted_distances[len / 2] as f64
    } else {
        // If the list length is even, return the average of the two middle elements
        let mid1 = sorted_distances[len / 2 - 1] as f64;
        let mid2 = sorted_distances[len / 2] as f64;
        (mid1 + mid2) / 2.0
    }
}


#[cfg(test)]
#[test]
mod tests{
    use super::*;
    // A function to get a subset of data from the Facebook data file
    // You can adjust this function to read a specific portion of the file
   fn test_sum() {
    let file = File::open("facebook_combined.txt").expect("Could not open file");
    let buf_reader = BufReader::new(file);
    
    let mut edges = Vec::new();
    let mut unique_nodes = HashSet::new();
    
    // Read first few lines as a sample subset
    for line in buf_reader.lines().take(100) {
        let line_str = line.expect("Error reading line");
        let v: Vec<&str> = line_str.trim().split_whitespace().collect();
        let x = v[0].parse::<usize>().expect("Failed to parse vertex");
        let y = v[1].parse::<usize>().expect("Failed to parse vertex");
        edges.push((x, y));
        unique_nodes.insert(x);
        unique_nodes.insert(y);
    }
    
    let num_unique = unique_nodes.len();
            
    // Create adjacency list from edges and unique nodes
    let unique_nodes = (0..num_unique).collect();
    let adj_list = adjacency_list(edges, unique_nodes);
    
    // You can define your expected adjacency list here based on the sample data
    // For now, just checking if the function runs without any issue
    assert!(!adj_list.is_empty());

    
// Test for `calculate_graph_statistics` function using the sample subset of Facebook data
    
    let graph = Graph { n: num_unique, outedges: adj_list };
    
    // Calculate graph statistics from vertex 0
    let (mean_dist, max_dist, median_dist) = calculate_graph_statistics(&graph, 0);
    
            // You can define expected mean, max, and median distances based on the sample data
            // For now, just checking if the function runs without any issue
    assert!(mean_dist >= 0.0);
    assert!(max_dist > 0);
    assert!(median_dist >= 0.0);


// Test for `compute_and_print_distance_bfs` function
    // Expected distances from vertex 0
    let mut computed_distances = vec![None; graph.n];
    compute_distances(0, &graph, &mut computed_distances);
    
    // Define expected distances based on the sample data
    // For now, just checking if the function runs without any issue
    assert!(!computed_distances.is_empty());
    }
}