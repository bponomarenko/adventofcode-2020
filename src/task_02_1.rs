use regex::Regex;

pub fn run(input: String) {
    let re = Regex::new(r"^(\d+)-(\d+)\s(.):\s(\w+)").unwrap();

    let count = input
        .split_terminator('\n')
        // Filter only strings with passwords, which satisfy requirements
        .filter(|s| {
            // Parse rule for policy specs and password
            let captures = re.captures(s).expect("Failed to capture string groups");
            let min: u8 = captures.get(1).unwrap().as_str().parse().expect("Min char should be a number");
            let max: u8 = captures.get(2).unwrap().as_str().parse().expect("Max char should be a number");
            let char = captures.get(3).unwrap().as_str();
            let password = captures.get(4).unwrap().as_str();

            // Calculate how many times "char" is present in the password word
            let matches_count = password.matches(&char).count() as u8;
            matches_count >= min && matches_count <= max
        })
        .count();

    println!("Result: {}", count);
}
