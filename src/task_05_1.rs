pub fn run(input: String) {
    println!("Result: {}", input.split_whitespace().map(|s| parse_id(s)).max().unwrap());
}

fn parse_id(line: &str) -> u32 {
    let bin = line.replace("F", "0").replace("L", "0").replace("B", "1").replace("R", "1");
    isize::from_str_radix(&bin, 2).unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::parse_id;

    #[test]
    fn should_convert_binary_to_int() {
        assert_eq!(parse_id("FBFBBFFRLR"), 357);
    }
}
