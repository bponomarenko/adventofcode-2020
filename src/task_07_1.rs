use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"^(?P<name>.+) bags contain (no other bags|(?P<bags>.+))\.$").unwrap();
    static ref BAGS_RE: Regex = Regex::new(r"^\d+ (.+) bags?$").unwrap();
}

pub fn run(input: String) {
    println!("Result: {}", count_bag_colors(input));
}

fn count_bag_colors(input: String) -> usize {
    let parent_bags_map = get_bags_map(&input);
    let mut unique_bags = HashSet::new();
    let mut queue = vec!["shiny gold"];

    while let Some(parent_bag) = queue.pop() {
        match parent_bags_map.get(parent_bag) {
            None => (),
            Some(parent_bags) => parent_bags.iter().for_each(|p| {
                queue.push(p);
                unique_bags.insert(p);
            }),
        }
    }

    unique_bags.len()
}

fn get_bags_map(input: &String) -> HashMap<&str, Vec<&str>> {
    let mut parent_bags_map = HashMap::new();

    input.split_terminator('\n').for_each(|line| {
        let caps = RULE_RE.captures(line).unwrap();
        match caps.name("bags") {
            None => (),
            Some(m) => m.as_str().split_terminator(", ").for_each(|b| {
                parent_bags_map
                    .entry(BAGS_RE.captures(b).unwrap().get(1).unwrap().as_str())
                    .or_insert(Vec::new())
                    .push(caps.name("name").unwrap().as_str().trim());
            }),
        }
    });
    parent_bags_map
}

#[cfg(test)]
mod tests {
    use super::count_bag_colors;

    #[test]
    fn should_correctly_count_bag_colors() {
        let input = String::from(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.",
        );
        assert_eq!(count_bag_colors(input), 4);
    }
}
