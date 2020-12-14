use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref MEM_RE: Regex = Regex::new(r"^mem\[(?P<addr>\d+)\] = (?P<value>\d+)$").unwrap();
}

pub fn run(input: String) -> u64 {
    let mut mask: String = String::new();
    let mut mem = HashMap::new();

    input.split_terminator('\n').for_each(|line| {
        if line.starts_with("mask") {
            mask = line.replace("mask = ", "").to_string();
        } else {
            let caps = MEM_RE.captures(line).unwrap();
            let addr = caps
                .name("addr")
                .unwrap()
                .as_str()
                .parse::<u64>()
                .expect("Mem address should be a string");
            let value = caps.name("value").unwrap().as_str().parse::<u64>().expect("Mem value should be a string");

            let mut and_mask: u64 = 0;
            let mut or_mask: u64 = 0;
            let mut variation_num: u64 = 1 << 36;
            let mut variations = vec![];

            mask.chars().for_each(|bit| {
                and_mask <<= 1;
                or_mask <<= 1;
                variation_num >>= 1;

                if bit == '1' {
                    or_mask |= 1;
                }
                if bit == 'X' {
                    variations.push(variation_num);
                } else {
                    and_mask |= 1;
                }
            });

            let base = addr & and_mask | or_mask;
            get_sum_options(&variations).iter().for_each(|option| {
                mem.insert(base + option, value);
            });
        }
    });
    mem.values().sum()
}

fn get_sum_options(variations: &Vec<u64>) -> Vec<u64> {
    if variations.len() == 1 {
        vec![0, variations[0]]
    } else {
        let sum_options = get_sum_options(&variations[1..].to_vec());
        sum_options
            .clone()
            .into_iter()
            .map(|v| v + variations[0])
            .chain(sum_options.into_iter())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input =
            String::from("mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1");
        assert_eq!(run(input), 208);
    }
}
