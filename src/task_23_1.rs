pub fn run(input: String) -> usize {
    1usize
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from("any");
        assert_eq!(run(input), 1);
    }
}
