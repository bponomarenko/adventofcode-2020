use itertools::Itertools;

pub fn run(input: String) -> u32 {
    let matching_combo = input
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Input should consist of numbers"))
        .combinations(2)
        .find(|combo: &Vec<u32>| combo.iter().sum::<u32>() == 2020);

    match matching_combo {
        None => 0,
        Some(combo) => combo.iter().product::<u32>(),
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from("1721 979 366 299 675 1456");
        assert_eq!(run(input), 514579);
    }
}
