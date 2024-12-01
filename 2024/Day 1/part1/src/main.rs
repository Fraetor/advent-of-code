use regex::Regex;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut left_vec = Vec::new();
    let mut right_vec = Vec::new();

    let re = Regex::new(r"([0-9]+)\s+([0-9]+)").unwrap();

    let lines = read_lines("../input").unwrap();
    for line in lines.flatten() {
        println!("> {}", line);
        let Some(caps) = re.captures(&line) else {
            panic!("No matches found!")
        };
        let left_location_id: i32 = caps[1].parse().unwrap();
        let right_location_id: i32 = caps[2].parse().unwrap();
        left_vec.push(left_location_id);
        right_vec.push(right_location_id);
    }
    println!("{:?} | {:?}", left_vec, right_vec);
    // Sort the vectors.
    left_vec.sort();
    right_vec.sort();
    println!("{:?} | {:?}", left_vec, right_vec);
    let mut distances = Vec::new();
    for index in 0..left_vec.len() {
        let distance = (left_vec[index] - right_vec[index]).abs();
        distances.push(distance);
    }
    println!("Distances: {:?}", distances);
    let total_distance = distances.into_iter().sum::<i32>();
    println!("Total distance: {}", total_distance);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
