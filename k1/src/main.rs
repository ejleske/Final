use std::env::args;
use std::error::Error;
use std::process;

use serde::Deserialize;

#[allow(dead_code, non_snake_case)]
#[derive(Debug, Deserialize)]
struct UserData {
    age: i32,
    gender: String,
    hours_on_social_media: i32,
    social_media: String,
    interest: String,
    profession: String,
    income: i32,
    indebt: bool,
    house_owner: bool,
    car_owner: bool,
}

fn example(path: &str, hh: bool) -> Result<(), Box<dyn Error>> {
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

fn main() {
    let file = "social_media.csv";
    let path: &str = &args[1];
    let has_headers: &str = &args[2];
    let mut hh = false;
    if has_headers == "true" {
       hh = true;
    }
    if let Err(err) = example(path, hh) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
