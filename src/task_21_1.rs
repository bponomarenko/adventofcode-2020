use std::collections::{hash_map::Entry, HashMap, HashSet};

fn parse_input(input: &String) -> Vec<(HashSet<&str>, HashSet<&str>)> {
    input
        .split_terminator("\n")
        .map(|food| {
            let food_parts: Vec<&str> = food[..food.len() - 1].split_terminator(" (contains ").collect();
            let ingridients: HashSet<&str> = food_parts.get(0).unwrap().split_whitespace().collect();
            let allergens: HashSet<&str> = food_parts.get(1).unwrap().split_terminator(", ").collect();
            (ingridients, allergens)
        })
        .collect()
}

pub fn run(input: String) -> u32 {
    let food_list = parse_input(&input);
    let mut possible_allergens = HashMap::new();
    let mut usage_count = HashMap::new();

    food_list.iter().for_each(|(ingridients, allergens)| {
        ingridients
            .iter()
            .for_each(|&ingridient| *usage_count.entry(String::from(ingridient)).or_insert(0) += 1);

        allergens.iter().for_each(|&allergen| {
            match possible_allergens.entry(allergen) {
                Entry::Occupied(mut entry) => {
                    let intersection: HashSet<&str> = ingridients.intersection(entry.get()).cloned().collect();
                    *entry.get_mut() = intersection;
                }
                Entry::Vacant(entry) => {
                    entry.insert(ingridients.clone());
                }
            };
        });
    });

    possible_allergens.iter().for_each(|(_, ingridients)| {
        ingridients.iter().for_each(|&ingridient| {
            usage_count.remove(ingridient);
        })
    });

    usage_count.values().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)");
        assert_eq!(run(input), 5);
    }
}
