
use std::env;
use std::error::Error;
use std::process;
use serde::Deserialize;

#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]

struct UserData {
    age: i32,
    gender: String,
    time_spent: i32,
    platform: String,
    interests: String,
    profession: String,
    income: i32,
    indebt: String,
    isHomeOwner: String,
    Owns_Car: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Check that the user has provided the correct number of arguments.
    if args.len() < 3 {
        eprintln!("Usage: {} <file_path> <has_headers>", args[0]);
        process::exit(1);
    }

    let path = &args[1];
    let has_headers = match args[2].as_str() {
        "true" => true,
        "false" => false,
        _ => {
            eprintln!("Invalid value for has_headers. Use 'true' or 'false'.");
            process::exit(1);
        }
    };

    // Run the example function and handle errors.
    if let Err(err) = read_csv(path, has_headers) {
        eprintln!("Error running example: {}", err);
        process::exit(1);
    }
}

fn read_csv(path: &str, hh: bool) -> Result<(), Box<dyn Error>> {
    println!("Using file {}", path);
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(hh)
        .flexible(true)
        .from_path(path)?;
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: UserData = result?;
        println!("{:?}", record);
    }
    Ok(())
}

