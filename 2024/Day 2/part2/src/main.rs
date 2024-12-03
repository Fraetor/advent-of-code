use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let records = load_data("../input");
    let mut safe_records = 0;

    for record in records {
        let record_description = if validate_record(&record) {
            safe_records += 1;
            "safe"
        } else {
            "unsafe"
        };
        println!("Record {record:?} is {record_description}.");
    }
    println!("Total safe records: {safe_records}");
}

/// Loads the data into a pair of vectors, one for each column.
fn load_data(filename: &str) -> Vec<Vec<i32>> {
    let mut records = Vec::new();
    let re = Regex::new(r"[0-9]+").unwrap();
    for line in read_lines(filename).unwrap().flatten() {
        let levels: Vec<_> = re
            .find_iter(&line)
            .map(|m| m.as_str().parse::<i32>().unwrap())
            .collect();
        records.push(levels);
    }
    return records;
}

fn validate_record(record: &Vec<i32>) -> bool {
    validate_record_strict(record) || validate_record_dampened(record)
}

/// A report only counts as safe if both of the following are true:
/// • The levels are either all increasing or all decreasing.
/// • Any two adjacent levels differ by at least one and at most three.
fn validate_record_strict(record: &Vec<i32>) -> bool {
    if record.len() < 2 {
        panic!("Must have at least 2 levels in a record.")
    }
    let increasing = record[0] < record[1];
    for index in 1..record.len() {
        let a = record[index - 1];
        let b = record[index];

        // Reject if don't differ.
        if a == b {
            return false;
        }

        // Reject if difference is larger than three.
        if a.abs_diff(b) > 3 {
            return false;
        }

        // Reject if direction of change is different from first pair.
        if (a < b) != increasing {
            return false;
        }
    }
    return true;
}

/// Try validating a bunch of records with a single level removed.
fn validate_record_dampened(record: &Vec<i32>) -> bool {
    for i in 0..record.len() {
        let mut temp_record = record.clone();
        temp_record.remove(i);
        if validate_record_strict(&temp_record) {
            return true;
        }
    }
    return false;
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
