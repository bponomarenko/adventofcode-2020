pub fn run(input: &String) {
    let count = input
        .split_terminator('\n')
        .filter(|s| {
            let partials: Vec<&str> = s.split_whitespace().collect();
            let char = partials[1].chars().nth(0).unwrap();
            let min_max: Vec<u8> = partials[0]
                .split('-')
                .map(|s| s.parse::<u8>().expect("Min-max limit should be defined with numbers"))
                .collect();

            let min_char = partials[2].chars().nth(min_max[0] as usize - 1).unwrap();
            let max_char = partials[2].chars().nth(min_max[0] as usize - 1).unwrap();
            char.eq(&min_char) | char.eq(&max_char)
        })
        .count();

    println!("Result: {}", count);
}
