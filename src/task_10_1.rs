pub fn run(input: String) -> usize {
    let mut data: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Input should consist of numbers"))
        .collect();
    data.sort();
    data.insert(0, 0);

    let mut diffs: (u32, u32) = (0, 1);

    for i in 1..data.len() {
        match data[i] - data[i - 1] {
            3 => diffs.1 += 1,
            1 => diffs.0 += 1,
            _ => (),
        }
    }
    (diffs.0 * diffs.1) as usize
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        assert_eq!(run(String::from("16 10 15 5 1 11 7 19 6 12 4")), 35);
        assert_eq!(
            run(String::from(
                "28 33 18 42 31 14 46 20 48 47 24 23 49 45 19 38 39 11 1 32 25 35 8 17 7 9 4 2 34 10 3"
            )),
            220
        );
    }
}
