use std::env;
use std::fs;
use std::path::Path;
use std::time::Instant;

#[path = "task_21_2.rs"]
mod task;

fn main() {
    println!("Reading input.txt");
    let input = read_input();
    println!("Processing...");

    let now = Instant::now();
    let res = task::run(input);
    println!("Result: {}", res);
    println!("Finished in: {:?}", now.elapsed());
}

fn read_input() -> String {
    let path = env::current_dir().expect("Cannot get current directory").join(Path::new("input.txt"));
    fs::read_to_string(path).expect("Unable to read input.txt file")
}
