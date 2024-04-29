use std::vec;
type Vertex = usize; // Define Vertex as usize
use std::collections::VecDeque;
use crate::Graph;

// Function to calculate graph statistics (mean, max, median distances) from a starting vertex
pub fn calculate_graph_statistics(graph: &Graph, start_vertex: Vertex) -> (f64, u32, f64) {
    let mut distances = vec![None; graph.n];
    distances_bfs(start_vertex, graph, &mut distances);

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