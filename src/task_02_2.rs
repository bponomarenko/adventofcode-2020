use regex::Regex;

pub fn run(input: &String) {
    let re = Regex::new(r"^(\d+)-(\d+)\s(.):\s(\w+)").unwrap();

    let count = input
        .split_terminator('\n')
        // Filter only strings with passwords, which satisfy requirements
        .filter(|s| {
            // Parse rule for policy specs and password
            let captures = re.captures(s).expect("Failed to capture string groups");
            let min: u8 = captures.get(1).unwrap().as_str().parse().expect("Min char should be a number");
            let max: u8 = captures.get(2).unwrap().as_str().parse().expect("Max char should be a number");
            let char = captures.get(3).unwrap().as_str().chars().nth(0).unwrap();
            let password = captures.get(4).unwrap().as_str();

            let min_char = password.chars().nth(min as usize - 1).unwrap();
            let max_char = password.chars().nth(max as usize - 1).unwrap();
            // Use XOR to determine when only one of two chars is matching
            min_char.eq(&char) ^ max_char.eq(&char)
        })
        .count();

    println!("Result: {}", count);
}
