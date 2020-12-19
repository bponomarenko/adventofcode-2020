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
            if *rule_id == 11 {
                // Special case for rule #11 with only one combo
                let combo = combos.get(0).unwrap();
                let a = combo.get(0).unwrap();
                let b = combo.get(1).unwrap();

                return format!(
                    "{0}({0}({0}({0}{1})?{1})?{1})?{1}",
                    assemble_regex_str(rules, a),
                    assemble_regex_str(rules, b)
                );
            }

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

            if *rule_id == 8 {
                // Special case for rule #8
                format!("({})+", regex_str)
            } else if combos.len() == 1 {
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
    fn should_run_correctly1() {
        let input =
            String::from("0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb");
        assert_eq!(run(input), 2);
    }

    #[test]
    fn should_run_correctly2() {
        let input =
            String::from("42: 9 14 | 10 1\n9: 14 27 | 1 26\n10: 23 14 | 28 1\n1: \"a\"\n11: 42 31\n5: 1 14 | 15 1\n19: 14 1 | 14 14\n12: 24 14 | 19 1\n16: 15 1 | 14 14\n31: 14 17 | 1 13\n6: 14 14 | 1 14\n2: 1 24 | 14 4\n0: 8 11\n13: 14 3 | 1 12\n15: 1 | 14\n17: 14 2 | 1 7\n23: 25 1 | 22 14\n28: 16 1\n4: 1 1\n20: 14 14 | 1 15\n3: 5 14 | 16 1\n27: 1 6 | 14 18\n14: \"b\"\n21: 14 1 | 1 14\n25: 1 1 | 1 14\n22: 14 14\n8: 42\n26: 14 22 | 1 20\n18: 15 15\n7: 14 5 | 1 21\n24: 14 1\n\n\nabbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\nbbabbbbaabaabba\nbabbbbaabbbbbabbbbbbaabaaabaaa\naaabbbbbbaaaabaababaabababbabaaabbababababaaa\nbbbbbbbaaaabbbbaaabbabaaa\nbbbababbbbaaaaaaaabbababaaababaabab\nababaaaaaabaaab\nababaaaaabbbaba\nbaabbaaaabbaaaababbaababb\nabbbbabbbbaaaababbbbbbaaaababb\naaaaabbaabaaaaababaa\naaaabbaaaabbaaa\naaaabbaabbaaaaaaabbbabbbaaabbaabaaa\nbabaaabbbaaabaababbaabababaaab\naabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba");
        assert_eq!(run(input), 12);
    }
}
