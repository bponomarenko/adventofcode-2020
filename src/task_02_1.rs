pub fn run(input: &String) {
    let count = input
        .split_terminator('\n')
        .filter(|s| {
            let partials: Vec<&str> = s.split_whitespace().collect();
            let char = partials[1].replace(":", "");
            let matches_count = partials[2].matches(&char).count() as u8;

            let min_max: Vec<u8> = partials[0]
                .split('-')
                .map(|s| s.parse::<u8>().expect("Min-max limit should be defined with numbers"))
                .collect();

            matches_count >= min_max[0] && matches_count <= min_max[1]
        })
        .count();

    println!("Result: {}", count);
}
