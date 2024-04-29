use rand::Rng;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::env;
use std::process;
use csv::ReaderBuilder;

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
struct UserData {
    age: i32,
    gender: String,
    time_spent: i32,
    platform: String,
    interests: String,
    profession: String,
    income: i32,
}

fn read_csv(file_path: &str, has_headers: bool) -> Result<Vec<UserData>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(has_headers)
        .from_path(file_path)?;

    let mut records = Vec::new();
    for result in rdr.deserialize() {
        let record: UserData = result?;
        records.push(record);
    }
    Ok(records)
}

fn pagerank(current: usize, data: &[UserData], user_map: &HashMap<UserData, usize>) -> usize {
    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(0..100);

    if random_num < 10 {
        // Randomly jump to a different user (node)
        let random_index = rng.gen_range(0..data.len());
        return random_index;
    } else {
        // Randomly select a neighbor to jump to
        let neighbors = find_neighbors(&data[current], data);
        if neighbors.is_empty() {
            return current; // Return current if no neighbors
        }
        let random_neighbor_index = rng.gen_range(0..neighbors.len());
        let neighbor = &neighbors[random_neighbor_index];
        *user_map.get(neighbor).unwrap_or(&current)
    }
}

fn walks(data: &Vec<UserData>) -> Result<Vec<f64>, Box<dyn Error>> {
    let n = data.len();
    let mut final_destination = vec![0.0; n];
    let user_map = map_users_to_indices(data)?;

    // Perform 100 iterations of the walk for each user
    for start in 0..n {
        let mut location = start;

        // Perform 100 walks from the starting location
        for _ in 0..100 {
            // Perform 100 steps in each walk
            for _ in 0..100 {
                location = pagerank(location, data, &user_map);
            }

            // Increment the count of the final destination
            final_destination[location] += 1.0;
        }
    }

    // Normalize the final destination counts
    for count in final_destination.iter_mut() {
        *count /= 100.0 * n as f64;
    }

    Ok(final_destination)
}

fn map_users_to_indices(data: &[UserData]) -> Result<HashMap<UserData, usize>, Box<dyn Error>> {
    let mut user_index_map = HashMap::new();

    for (index, user) in data.iter().enumerate() {
        user_index_map.insert(user.clone(), index);
    }

    Ok(user_index_map)
}

fn find_neighbors(user: &UserData, data: &[UserData]) -> Vec<UserData> {
    let mut neighbors = Vec::new();

    for other_user in data {
        // Check if the other user is a potential neighbor
        if (user.age - other_user.age).abs() <= 10
            && user.gender == other_user.gender
            && (user.time_spent - other_user.time_spent).abs() <= 0_5
        {
            neighbors.push(other_user.clone());
        }
    }

    neighbors
}


fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check for minimum number of arguments
    if args.len() < 3 {
        eprintln!("Usage: {} <file_path> <has_headers>", args[0]);
        process::exit(1);
    }

    // Extract file path and headers flag from arguments
    let file_path = &args[1];
    let has_headers = match args[2].as_str() {
        "true" => true,
        "false" => false,
        _ => {
            eprintln!("Invalid value for has_headers. Use 'true' or 'false'.");
            process::exit(1);
        }
    };

    // Read data from the CSV file
    let data = read_csv(file_path, has_headers);
    
    // Proceed only if data is successfully loaded
    if let Ok(data) = data {
        // Select a user to find the closest neighbors (you can modify this to choose the user dynamically)
        let selected_user_index = 0; // Change this index as per requirement
        let user = &data[selected_user_index];

        // Find the 10 closest neighbors for the selected user
        let closest_neighbors = closest_neighbors(user, &data);

        // Print the 10 closest neighbors and their distances
        println!("10 closest neighbors for User {}:", selected_user_index);
        for (neighbor, distance) in closest_neighbors {
            println!("Neighbor: {:?}, Distance: {:.2}", neighbor, distance);
        }

        // Perform random walks and compute final destination probabilities
        let final_destination = walks(&data);
        
        // Print final destination probabilities
        println!("Final destination probabilities:");
        for (index, probability) in final_destination.iter().enumerate() {
            println!("User {}: {:.2}", index, probability);
        }
    } else {
        // Handle error when loading data
        eprintln!("Error reading CSV file: {}", data.unwrap_err());
        process::exit(1);
    }
}


fn distance(user1: &UserData, user2: &UserData) -> f64 {
    // Calculate Euclidean distance based on age, time spent, and other attributes
    let age_diff = (user1.age - user2.age).abs() as f64;
    let time_spent_diff = (user1.time_spent - user2.time_spent).abs() as f64;
    
    // Combine the differences into a single distance value
    // Using Euclidean distance as a metric
    ((age_diff.powi(2) + time_spent_diff.powi(2)).sqrt())
}

fn closest_neighbors(user: &UserData, data: &[UserData]) -> Vec<(UserData, f64)> {
    // Create a vector to hold distances
    let mut distances: Vec<(UserData, f64)> = Vec::new();
    
    // Calculate distances between the user and all other users in the data
    for other_user in data {
        if other_user != user {
            let distance = distance(user, other_user);
            distances.push((other_user.clone(), distance));
        }
    }
    
    // Sort the distances in ascending order based on the distance value
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    // Return the 10 closest neighbors
    distances.into_iter().take(10).collect()
}

// Usage example:
// let user: UserData = ...; // Define a user
// let data: Vec<UserData> = ...; // Define a dataset
// let closest_neighbors = find_closest_neighbors(&user, &data);
// for (neighbor, distance) in closest_neighbors {
//     println!("Neighbor: {:?}, Distance: {:.2}", neighbor, distance);
// }