use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref RULE_RE: Regex = Regex::new(r"^(?P<name>.+) bags contain (no other bags|(?P<bags>.+))\.$").unwrap();
    static ref BAGS_RE: Regex = Regex::new(r"^(?P<count>\d+) (?P<name>.+) bags?$").unwrap();
}

pub fn run(input: String) {
    println!("Result: {}", count_bag_colors(input));
}

fn count_bag_colors(input: String) -> u32 {
    let bag_content_map = get_bags_map(&input);
    count_bags_recursive(
        &bag_content_map,
        &Bag {
            name: String::from("shiny gold"),
            count: 0,
        },
    )
}

fn count_bags_recursive(bag_content_map: &HashMap<&str, Vec<Bag>>, bag: &Bag) -> u32 {
    match bag_content_map.get(bag.name.as_str()) {
        None => 0,
        Some(containing_bags) => containing_bags
            .iter()
            .fold(0, |acc, p| acc + p.count * (1 + count_bags_recursive(bag_content_map, &p))),
    }
}
#[derive(Debug)]
struct Bag {
    count: u32,
    name: String,
}

impl From<&str> for Bag {
    fn from(str: &str) -> Self {
        let caps = BAGS_RE.captures(str).unwrap();
        Bag {
            count: caps.name("count").unwrap().as_str().parse::<u32>().unwrap(),
            name: caps.name("name").unwrap().as_str().to_owned(),
        }
    }
}

fn get_bags_map(input: &String) -> HashMap<&str, Vec<Bag>> {
    let mut bag_map = HashMap::new();

    input.split_terminator('\n').for_each(|line| {
        let caps = RULE_RE.captures(line).unwrap();
        match caps.name("bags") {
            None => (),
            Some(m) => m.as_str().split_terminator(", ").for_each(|b| {
                bag_map
                    .entry(caps.name("name").unwrap().as_str().trim())
                    .or_insert(Vec::new())
                    .push(Bag::from(b));
            }),
        }
    });
    bag_map
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
        assert_eq!(count_bag_colors(input), 32);
    }

    #[test]
    fn should_correctly_count_bag_colors2() {
        let input = String::from(
            "shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.",
        );
        assert_eq!(count_bag_colors(input), 126);
    }
}
