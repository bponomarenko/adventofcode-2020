use std::collections::HashSet;
use std::iter::FromIterator;

pub fn run(input: String) -> usize {
    let alphabet: HashSet<char> = HashSet::from_iter((b'a'..=b'z').into_iter().map(|c| c as char));
    input
        .split_terminator("\n\n")
        .map(|g| {
            g.split_whitespace()
                .map(|s| HashSet::from(s.chars().collect::<HashSet<char>>()))
                .fold(alphabet.clone(), |acc, h| HashSet::from_iter(acc.intersection(&h).map(|c| *c)))
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_do_count_correctly() {
        assert_eq!(run(String::from("abcx\nabcy\nabcz")), 3);
        assert_eq!(run(String::from("abc\n\na\nb\nc\n\nab\nac\n\na\na\na\na\n\nb")), 6);
    }
}
