use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::ops::RangeInclusive;

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"^(?P<field>.+): (?P<min1>\d+)-(?P<max1>\d+) or (?P<min2>\d+)-(?P<max2>\d+)$").unwrap();
}

struct Rule {
    range1: RangeInclusive<u32>,
    range2: RangeInclusive<u32>,
}

fn get_num_capture(caps: &Captures, name: &str) -> u32 {
    caps.name(name)
        .unwrap()
        .as_str()
        .parse::<u32>()
        .expect("Range value is not valid integer")
}

impl Rule {
    fn from_str(str: &str) -> Self {
        let caps = RULE_RE.captures(str).expect("Rule definition is invalid");
        Rule {
            range1: RangeInclusive::new(get_num_capture(&caps, "min1"), get_num_capture(&caps, "max1")),
            range2: RangeInclusive::new(get_num_capture(&caps, "min2"), get_num_capture(&caps, "max2")),
        }
    }

    fn is_value_valid(&self, value: &u32) -> bool {
        self.range1.contains(value) || self.range2.contains(value)
    }
}

struct Ticket {
    values: Vec<u32>,
}

impl Ticket {
    fn from_str(str: &str) -> Self {
        Ticket {
            values: str
                .split_terminator(",")
                .map(|value| value.parse::<u32>().expect("Ticket value is invalid"))
                .collect(),
        }
    }

    fn get_invalid_values_sum(&self, rules: &Vec<Rule>) -> u32 {
        self.values
            .iter()
            .filter(|&value| rules.iter().all(|rule| !rule.is_value_valid(value)))
            .sum()
    }
}

fn parse_input(input: String) -> (Vec<Rule>, Vec<Ticket>) {
    let raw_input = input.split_terminator("\n\n").collect::<Vec<&str>>();
    match raw_input.as_slice() {
        [raw_rules, .., raw_tickets] => {
            let rules = raw_rules.split_terminator("\n").map(|r| Rule::from_str(r)).collect();
            let tickets = raw_tickets
                .split_terminator("\n")
                .skip(1)
                .map(|ticket| Ticket::from_str(ticket))
                .collect();
            (rules, tickets)
        }
        _ => panic!("Input is invalid"),
    }
}

pub fn run(input: String) -> u32 {
    let (rules, tickets) = parse_input(input);
    tickets.iter().map(|ticket| ticket.get_invalid_values_sum(&rules)).sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from("class: 1-3 or 5-7\nrow: 6-11 or 33-44\nseat: 13-40 or 45-50\n\nyour ticket:\n7,1,14\n\nnearby tickets:\n7,3,47\n40,4,50\n55,2,20\n38,6,12");
        assert_eq!(run(input), 71);
    }
}
