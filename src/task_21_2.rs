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

pub fn run(input: String) -> String {
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

    let mut allergens = HashMap::new();

    while possible_allergens.len() > 0 {
        possible_allergens.iter().for_each(|(allergen, ingridients)| {
            // All the ingridients in question
            let unknown_ingridients: Vec<&str> = ingridients
                .iter()
                .filter(|&ingridient| !allergens.contains_key(ingridient))
                .cloned()
                .collect();
            if unknown_ingridients.len() == 1 {
                allergens.insert(*unknown_ingridients.get(0).unwrap(), *allergen);
            }
        });

        // Cleanup resolved allergens
        allergens.values().for_each(|allergen| {
            possible_allergens.remove(allergen);
        });
    }

    let mut dangerous_ingridients: Vec<(&str, &str)> = allergens.iter().map(|(&ingridient, &allergen)| (ingridient, allergen)).collect();
    dangerous_ingridients.sort_by(|(_, a1), (_, a2)| a1.cmp(a2));

    // Build and return canonical dangerous ingredient list
    dangerous_ingridients
        .iter()
        .map(|(ingridient, _)| ingridient.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\ntrh fvjkl sbzzf mxmxvkd (contains dairy)\nsqjhc fvjkl (contains soy)\nsqjhc mxmxvkd sbzzf (contains fish)");
        assert_eq!(run(input), String::from("mxmxvkd,sqjhc,fvjkl"));
    }
}
