use regex::Regex;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let (mut left, mut right) = load_data("../input");
    // println!("{:?} | {:?}", left, right);

    let total_distance = sorted_difference(&mut left, &mut right);
    println!("Total distance: {}", total_distance);

    let similarity_score = similarity_score(&left, &right);
    println!("Similarity score: {}", similarity_score);
}

/// Loads the data into a pair of vectors, one for each column.
fn load_data(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    let re = Regex::new(r"([0-9]+)\s+([0-9]+)").unwrap();

    let lines = read_lines(filename).unwrap();
    for line in lines.flatten() {
        let Some(caps) = re.captures(&line) else {
            panic!("No matches found!")
        };
        left.push(caps[1].parse().unwrap());
        right.push(caps[2].parse().unwrap());
    }
    return (left, right);
}

/// The sum of the difference between the elements after sorting each vector and
/// taking the difference between them. The vectors should have the same length.
fn sorted_difference(left: &mut Vec<i32>, right: &mut Vec<i32>) -> u32 {
    // Check invariant.
    if left.len() != right.len() {
        panic!(
            "left and right must be the same length. Got {} and {}.",
            left.len(),
            right.len()
        )
    }

    // Sort vectors.
    left.sort();
    right.sort();

    // Calculate distances.
    let mut distances = Vec::with_capacity(left.len());
    for index in 0..left.len() {
        distances.push(i32::abs_diff(left[index], right[index]));
    }
    return distances.into_iter().sum::<u32>();
}

/// The similarity score comes from adding up each number in the left list after
/// multiplying it by the number of times that number appears in the right list.
fn similarity_score(left: &Vec<i32>, right: &Vec<i32>) -> i32 {
    // Count how many of each number we have in the right list.
    let mut rhs_count = BTreeMap::new();
    for location_id in right {
        *rhs_count.entry(location_id).or_insert(0) += 1;
    }

    // Calculate similarity score.
    let mut total = 0;
    for location_id in left {
        total += location_id * rhs_count.get(location_id).unwrap_or(&0);
    }
    return total;
}

/// Read lines function from Rust by Example.
/// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
