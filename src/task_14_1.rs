use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref MEM_RE: Regex = Regex::new(r"^mem\[(?P<index>\d+)\] = (?P<value>\d+)$").unwrap();
}

pub fn run(input: String) -> u64 {
    let mut mask: String = String::new();
    let mut mem = HashMap::new();

    input.split_terminator('\n').for_each(|line| {
        if line.starts_with("mask") {
            mask = line.replace("mask = ", "").to_string();
        } else {
            let caps = MEM_RE.captures(line).unwrap();
            let value = caps.name("value").unwrap().as_str().parse::<u64>().expect("Mem value should be a string");

            let mut and_mask: u64 = 0;
            let mut or_mask: u64 = 0;
            mask.chars().for_each(|bit| {
                and_mask <<= 1;
                or_mask <<= 1;
                if bit == '1' {
                    or_mask |= 1;
                }
                if bit != '0' {
                    and_mask |= 1;
                }
            });
            mem.insert(caps.name("index").unwrap().as_str(), value & and_mask | or_mask);
        }
    });
    mem.values().sum()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0");
        assert_eq!(run(input), 165);
    }
}
