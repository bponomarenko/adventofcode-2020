pub fn run(input: String) -> u64 {
    let mut data: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Input should consist of numbers"))
        .collect();

    data.sort();
    // Add first and last element to the data
    data.insert(0, 0);
    data.push(data.iter().max().unwrap() + 3);

    let mut distance = 0;
    let mut res = 1;

    for i in 1..data.len() {
        match data[i] - data[i - 1] {
            1 => distance += 1,
            _ => {
                if distance > 1 {
                    res *= 2u64.pow(distance - 1) - (if distance > 3 { 1 } else { 0 });
                }
                distance = 0;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        assert_eq!(run(String::from("16 10 15 5 1 11 7 19 6 12 4")), 8);
        assert_eq!(
            run(String::from(
                "28 33 18 42 31 14 46 20 48 47 24 23 49 45 19 38 39 11 1 32 25 35 8 17 7 9 4 2 34 10 3"
            )),
            19208
        );
    }
}
