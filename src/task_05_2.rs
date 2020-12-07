pub fn run(input: String) {
    let mut ids_iter: Vec<u32> = input.split_whitespace().map(|s| parse_id(s)).collect();
    ids_iter.sort();

    let mut my_id = 0;
    for i in 1..ids_iter.len() {
        if ids_iter[i] - ids_iter[i - 1] > 1 {
            my_id = ids_iter[i] - 1;
            break;
        }
    }
    println!("Result: {}", my_id);
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
