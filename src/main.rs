use std::env;
use std::fs;
use std::path::Path;

#[path = "task_10_2.rs"]
mod task;

fn main() {
    let res = task::run(read_input());
    println!("Result: {}", res);
}

fn read_input() -> String {
    println!("Reading input.txt");
    let path = env::current_dir().expect("Cannot get current directory").join(Path::new("input.txt"));
    fs::read_to_string(path).expect("Unable to read input.txt file")
}
