use regex::Regex;
use std::collections::HashMap;

enum Rule {
    Char(char),
    Combo(Vec<Vec<u8>>),
}

type Rules = HashMap<u8, Rule>;

fn populate_rules(raw_rules: &str) -> Rules {
    let rule_re = Regex::new(r#"^(?P<id>\d+): ("(?P<char>[a-z])"|(?P<combo>\d+( (\d+|\|))*))$"#).unwrap();
    let mut rules: Rules = HashMap::new();

    raw_rules.split_terminator("\n").for_each(|rule| {
        let caps = rule_re.captures(rule).expect("Rule definition is not valid");
        let id = caps.name("id").expect("Rule should have numeric id").as_str().parse::<u8>().unwrap();

        if let Some(rule_char) = caps.name("char") {
            rules.insert(id, Rule::Char(rule_char.as_str().chars().nth(0).unwrap()));
        } else if let Some(rule_combo) = caps.name("combo") {
            let combo: Vec<Vec<u8>> = rule_combo
                .as_str()
                .split_terminator(" | ")
                .map(|combo| combo.split_whitespace().map(|id| id.parse::<u8>().unwrap()).collect())
                .collect();
            rules.insert(id, Rule::Combo(combo));
        }
    });
    rules
}

fn parse_input(input: &String) -> (Rules, Vec<&str>) {
    let raw_input = input.split_terminator("\n\n").collect::<Vec<&str>>();
    match raw_input.as_slice() {
        [rules_str, messages_str, ..] => (populate_rules(rules_str), messages_str.split_whitespace().collect()),
        _ => panic!("Input is invlaid"),
    }
}

fn assemble_regex_str(rules: &Rules, rule_id: &u8) -> String {
    match rules.get(rule_id) {
        Some(Rule::Char(rule_char)) => rule_char.to_string(),
        Some(Rule::Combo(combos)) => {
            let regex_str = combos
                .iter()
                .map(|combo| {
                    combo
                        .clone()
                        .iter()
                        .map(|id| assemble_regex_str(rules, id))
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect::<Vec<String>>()
                .join("|");

            if combos.len() == 1 {
                regex_str.to_string()
            } else {
                format!("({})", regex_str)
            }
        }
        _ => panic!("Should have rule by the specified id"),
    }
}

pub fn run(input: String) -> usize {
    let (rules, messages) = parse_input(&input);
    let valid_messages_regex_str = assemble_regex_str(&rules, &0);
    let valid_message_re = Regex::new(&format!("^{}$", valid_messages_regex_str)).unwrap();

    messages
        .iter()
        .filter(|message| valid_message_re.is_match(message))
        .collect::<Vec<&&str>>()
        .len()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input =
            String::from("0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb");
        assert_eq!(run(input), 2);
    }
}
