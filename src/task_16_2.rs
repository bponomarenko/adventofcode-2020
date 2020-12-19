use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::ops::RangeInclusive;

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"^(?P<field>.+): (?P<min1>\d+)-(?P<max1>\d+) or (?P<min2>\d+)-(?P<max2>\d+)$").unwrap();
}

#[derive(Debug, Clone)]
struct Rule {
    field: String,
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
            field: caps.name("field").unwrap().as_str().to_string(),
            range1: RangeInclusive::new(get_num_capture(&caps, "min1"), get_num_capture(&caps, "max1")),
            range2: RangeInclusive::new(get_num_capture(&caps, "min2"), get_num_capture(&caps, "max2")),
        }
    }

    fn is_value_valid(&self, value: &u32) -> bool {
        self.range1.contains(value) || self.range2.contains(value)
    }

    fn is_departure_field(&self) -> bool {
        self.field.starts_with("departure ")
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

    fn is_valid(&self, rules: &Vec<Rule>) -> bool {
        self.values.iter().all(|value| rules.iter().any(|rule| rule.is_value_valid(value)))
    }
}

fn parse_input(input: String) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let raw_input = input.split_terminator("\n\n").collect::<Vec<&str>>();
    match raw_input.as_slice() {
        [raw_rules, raw_my_ticket, raw_tickets] => {
            let rules = raw_rules.split_terminator("\n").map(|r| Rule::from_str(r)).collect();
            let tickets = raw_tickets
                .split_terminator("\n")
                .skip(1)
                .filter_map(|ticket_str| {
                    let ticket = Ticket::from_str(ticket_str);
                    if ticket.is_valid(&rules) {
                        Some(ticket)
                    } else {
                        None
                    }
                })
                .collect();
            let my_ticket = Ticket::from_str(raw_my_ticket.split_terminator("\n").nth(1).unwrap());

            (rules, my_ticket, tickets)
        }
        _ => panic!("Input is invalid"),
    }
}

fn find_valid_rule_set(rules: &Vec<Rule>, tickets: &Vec<Ticket>, value_index: usize) -> Option<Vec<Rule>> {
    let mut min_pos = 0;
    loop {
        let rule_pos = rules
            .iter()
            .enumerate()
            .position(|(i, rule)| i >= min_pos && tickets.iter().all(|ticket| rule.is_value_valid(&ticket.values[value_index])));

        match rule_pos {
            None => break None,
            Some(pos) => {
                let mut cloned_rules = rules.clone();
                let first_rule = cloned_rules.remove(pos);
                let mut res = vec![first_rule];

                if cloned_rules.is_empty() {
                    break Some(res);
                }

                match find_valid_rule_set(&cloned_rules, tickets, value_index + 1) {
                    Some(rule_set) => {
                        res.extend(rule_set);
                        break Some(res);
                    }
                    _ => min_pos = pos + 1,
                };
            }
        }
    }
}

pub fn run(input: String) -> u64 {
    let (rules, my_ticket, tickets) = parse_input(input);
    let correct_rule_set = find_valid_rule_set(&rules, &tickets, 0).expect("Input rules should contain valid combination");

    correct_rule_set
        .iter()
        .zip(my_ticket.values)
        .fold(1, |acc, (rule, value)| if rule.is_departure_field() { acc * (value as u64) } else { acc })
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from(
            "departure class: 0-1 or 4-19\ndeparture row: 0-5 or 8-19\ndeparture seat: 0-13 or 16-19\n\nyour ticket:\n11,12,13\n\nnearby tickets:\n3,9,18\n15,1,5\n5,14,9",
        );
        assert_eq!(run(input), 1716);
    }
}
