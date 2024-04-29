use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::vec;

type Vertex = usize; // Define Vertex as usize

#[derive(Debug)]
struct Graph {
    n: usize, // Number of vertices
    outedges: Vec<Vec<usize>>, // Adjacency list representation of the graph
}

fn main() {
    let path = "facebook_combined.txt"; // Change this path if necessary
    let (edges, n) = read_file(path); // Read the file and get edges and number of vertices

    let mut graph = Graph {
        n,
        outedges: vec![vec![]; n],
    };

    // Create the adjacency list representation of the graph
    for (v, w) in edges.iter() {
        graph.outedges[*v].push(*w);
        graph.outedges[*w].push(*v);
    }

    // Run BFS from the starting vertex (you can change this to any vertex you want)
    let start_vertex = 0;
    compute_and_print_distance_bfs(start_vertex, &graph);
}

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

    // Print distances from start to each vertex
    println!("Vertex: Distance from start vertex {}:", start);
    for v in 0..graph.n {
        match distance[v] {
            Some(dist) => println!("{}: {}", v, dist),
            None => println!("{}: Unreachable", v),
        }
    }

}


fn adjacency_list(edges:Vec<(usize,usize)>, unique_nodes: HashSet<usize>) -> Vec<Vec<usize>> {
    let num_unique = unique_nodes.len();
    let vec_unique_nodes: Vec<&usize> = unique_nodes.iter().collect();
    let mut graph_list: Vec<Vec<usize>> = vec![vec![]; num_unique];
    let mut node_map: HashMap<usize, usize> =  HashMap::new();

    for (idx, val) in vec_unique_nodes.iter().enumerate(){
        node_map.insert(**val, idx);
    }

    for 
}



// mean, mode, max, 