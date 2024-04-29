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
        if (user.age - other_user.age).abs() <= 5
            && user.gender == other_user.gender
            && (user.time_spent - other_user.time_spent).abs() <= 10
        {
            neighbors.push(other_user.clone());
        }
    }

    neighbors
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <file_path> <has_headers>", args[0]);
        process::exit(1);
    }

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

    // Perform random walks and compute final destination probabilities
    match walks(&data.unwrap()) {
        Ok(final_destination) => {
            println!("Final destination probabilities:");
            for (index, probability) in final_destination.iter().enumerate() {
                println!("User {}: {:.2}", index, probability);
            }
        }
        Err(err) => {
            eprintln!("Error computing walks: {}", err);
            process::exit(1);
        }
    }
}


