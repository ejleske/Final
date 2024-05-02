use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

mod bfs;
use crate::bfs::adjacency_list;
use crate::bfs::distances_bfs;
use crate::bfs::unique_nodes;

mod stats;
use crate::stats::calculate_graph_statistics;

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

    // Print the adjacency list (adjacency map)
    // can make the following into a comment so that it does not return the adjacecy list:
    println!("Adjacency map:");
    for (vertex, neighbors) in graph.outedges.iter().enumerate() {
         print!("Vertex {}: ", vertex);
         for &neighbor in neighbors {
             print!("{} ", neighbor);
        }
         println!();
    }
    // comment out until this point to remove the adjacency list
     
    // Calculate mean, max, and median distances
    let (mean_dist, max_dist, median_dist) = calculate_graph_statistics(&graph, start_vertex);

    // Print the results
    println!("Mean distance between different vertices: {:.2}", mean_dist);
    println!("Maximum distance between different vertices: {}", max_dist);
    println!("Median distance between different vertices: {:.2}", median_dist);


    let mut distances: Vec<Option<u32>> = vec![None; graph.n];
    distances_bfs(start_vertex, &graph, &mut distances);

    // Convert and sort distances
    // Convert distances from Option<u32> to a list of tuples (vertex index, distance)
    let mut distances_list: Vec<(usize, u32)> = distances.iter().enumerate()
    .filter_map(|(i, &dist)| dist.map(|d| (i, d)))
    .collect();

    // Sort the list by distance    
    distances_list.sort_by_key(|&(_, distance)| distance);

    // Step 3: Extract the 15 highest and lowest distances
    let num_distances = distances_list.len();
    let lowest_15 = &distances_list[..15]; // First 15 distances (lowest)
    let highest_15 = &distances_list[num_distances - 15..]; // Last 15 distances (highest)

    // Step 4: Print the lowest and highest distances
    println!("The 15 distances with the lowest values:");
    for &(index, distance) in lowest_15 {
        println!("Vertex {}: Distance {}", index, distance);
    }

    println!("The 15 distances with the highest values:");
    for &(index, distance) in highest_15.iter().rev() {
        println!("Vertex {}: Distance {}", index, distance);
    }
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
    distances_bfs(0, &graph, &mut computed_distances);
    
    // Define expected distances based on the sample data
    // For now, just checking if the function runs without any issue
    assert!(!computed_distances.is_empty());
    }
}