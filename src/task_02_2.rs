pub fn run(input: &String) {
    let count = input
        .split_terminator('\n')
        // Filter only strings with passwords, which satisfy requirements
        .filter(|s| {
            // Split into [min-max requirements, char, password]
            let partials: Vec<&str> = s.split_whitespace().collect();
            let char = partials[1].chars().nth(0).unwrap();
            // Parse "min-max" into [min, max]
            let min_max: Vec<u8> = partials[0]
                .split('-')
                .map(|s| s.parse::<u8>().expect("Min-max limit should be defined with numbers"))
                .collect();

            let min_char = partials[2].chars().nth(min_max[0] as usize - 1).unwrap();
            let max_char = partials[2].chars().nth(min_max[1] as usize - 1).unwrap();
            // Use XOR to determine when only one of two chars is matching
            min_char.eq(&char) ^ max_char.eq(&char)
        })
        .count();

    println!("Result: {}", count);
}
