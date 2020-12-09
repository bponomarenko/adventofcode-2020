pub fn run(input: String) -> usize {
    find_faulty_number(input, 25)
}

fn find_faulty_number(input: String, preamble: u8) -> usize {
    let sequence: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("Input should consist of numbers"))
        .collect();

    let faulty_num = sequence.iter().skip(preamble as usize).enumerate().find(|(i, &num)| {
        for x in &sequence[*i..*i + (preamble as usize)] {
            for y in &sequence[*i + 1..*i + (preamble as usize)] {
                if x + y == num {
                    return false;
                }
            }
        }
        // If none of the combinations sum equals to the number - that is the one
        true
    });

    match faulty_num {
        Some(n) => *n.1,
        None => panic!("Input sequence doesn't contain faulty number"),
    }
}

#[cfg(test)]
mod tests {
    use super::find_faulty_number;

    #[test]
    fn should_run_correctly() {
        let input = String::from("35 20 15 25 47 40 62 55 65 95 102 117 150 182 127 219 299 277 309 576");
        assert_eq!(find_faulty_number(input, 5), 127);
    }
}
