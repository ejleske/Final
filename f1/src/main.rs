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
    let bfs = compute_and_print_distance_bfs(start_vertex, &graph);

    // Print the adjacency list (adjacency map)
    println!("Adjacency map:");
    for (vertex, neighbors) in graph.outedges.iter().enumerate() {
        print!("Vertex {}: ", vertex);
        for &neighbor in neighbors {
            print!("{} ", neighbor);
        }
        println!();
    }

    // Mark and print components using BFS
    let mut component: Vec<Option<Component>> = vec![None; n];
    let mut component_no = 0;

    // Iterate through each vertex to mark and assign components
    for v in 0..n {
        if component[v].is_none() {
            // Start marking component for vertex v
            mark_component_bfs(v, &graph, &mut component, component_no);
            component_no += 1;
        }
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

    // Print distances from start vertex to each other vertex
    // println!("Vertex: Distance from start vertex {}:", start);
    // for v in 0..graph.n {
    //     match distance[v] {
    //         Some(dist) => println!("{}: {}", v, dist),
    //         None => println!("{}: Unreachable", v),
    //     }
    // }
}

// Function to mark components in the graph using BFS
fn mark_component_bfs(vertex: Vertex, graph: &Graph, component: &mut Vec<Option<Component>>, component_no: Component) {
    component[vertex] = Some(component_no);

    let mut queue = VecDeque::new();
    queue.push_back(vertex);

    while let Some(v) = queue.pop_front() {
        for &w in &graph.outedges[v] {
            if component[w].is_none() {
                component[w] = Some(component_no);
                queue.push_back(w);
            }
        }
    }
}
