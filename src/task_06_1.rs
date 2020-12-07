use itertools::Itertools;

pub fn run(input: String) -> usize {
    input
        .split_terminator("\n\n")
        .map(|g| g.chars().filter(|c| *c != '\n').unique().count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_do_count_correctly() {
        assert_eq!(run(String::from("abcx\nabcy\nabcz")), 6);
        assert_eq!(run(String::from("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb")), 11);
    }
}
