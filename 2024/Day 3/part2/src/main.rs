use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Command {
    // op: &'a str,
    arg1: i32,
    arg2: i32,
}

fn main() {
    let commands = load_commands("commands");
    let total: i32 = commands.iter().map(|c| executor(c)).sum();
    println!("Total: {total}");
}

fn load_commands(filename: &str) -> Vec<Command> {
    let mut commands = Vec::new();
    let mut enabled = true;
    let re = Regex::new(r"(\w)\(([0-9]+),([0-9]+)\)").unwrap();
    for line in read_lines(filename).unwrap().flatten() {
        if line == "do()" {
            enabled = true;
        } else if line == "don't()" {
            enabled = false;
        } else {
            if !enabled {
                continue;
            }
            println!("Processing {line}");
            let caps = re.captures(&line).unwrap();
            commands.push(Command {
                // op: &caps[1],
                arg1: caps[2].parse().unwrap(),
                arg2: caps[3].parse().unwrap(),
            });
        }
    }
    return commands;
}

fn executor(command: &Command) -> i32 {
    return command.arg1 * command.arg2;
    // match command.op {
    //     "mul" => multiply(command.arg1, command.arg2),
    //     _ => panic!("Unknown operation {}", command.op),
    // }
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
