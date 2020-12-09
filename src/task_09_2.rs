pub fn run(input: String) -> usize {
    find_encryption_weakness(input, 25)
}

fn find_faulty_number(sequence: &Vec<usize>, preamble: u8) -> usize {
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

fn find_encryption_weakness(input: String, preamble: u8) -> usize {
    let sequence: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().expect("Input should consist of numbers"))
        .collect();

    let faulty_num = find_faulty_number(&sequence, preamble);
    for i in 0..sequence.len() {
        let slice_iter = sequence.iter().skip(i);

        for len in 2..slice_iter.len() {
            let take_slice = slice_iter.clone().take(len);
            let sum = take_slice.clone().sum::<usize>();
            if sum == faulty_num {
                return take_slice.clone().min().unwrap_or(&0) + take_slice.clone().max().unwrap_or(&0);
            }
            if sum > faulty_num {
                break;
            }
        }
    }
    panic!("Should have found the sequence so far");
}

#[cfg(test)]
mod tests {
    use super::find_encryption_weakness;

    #[test]
    fn should_run_correctly() {
        let input = String::from("35 20 15 25 47 40 62 55 65 95 102 117 150 182 127 219 299 277 309 576");
        assert_eq!(find_encryption_weakness(input, 5), 62);
    }
}
