use std::collections::HashMap;

type Cups = HashMap<u64, Option<u64>>;

fn get_value(cups: &Cups, pos: &Option<u64>, level: u8) -> u64 {
    let mut value = pos;
    for _ in 0..=level {
        value = cups.get(&value.unwrap()).unwrap();
    }
    value.unwrap()
}

pub fn run(input: String) -> u64 {
    // Using VecDecue would be really slow as searching through it would take a lot of time
    // Use hash map to get item references faster, but this requires custom implementation of the connected node list
    let mut cups: Cups = HashMap::new();
    let mut first = None;
    let mut last = None;

    // Prefil cups map
    input.trim().chars().for_each(|cup| {
        let c = cup.to_digit(10).unwrap().into();
        if let None = first {
            first = Some(c);
        }

        if let Some(last_cup) = last {
            cups.insert(last_cup, Some(c));
        }
        last = Some(c);

        cups.insert(c, None);
    });

    // Add more items, up to a million O_o
    for i in 10..=1_000_000 {
        cups.insert(last.unwrap(), Some(i));
        cups.insert(i, None);
        last = Some(i);
    }

    // Play Carb Cups!!
    for _ in 0..10_000_000 {
        // Find next destination
        let mut destination = first.unwrap() - 1;
        while destination == 0
            || destination == get_value(&cups, &first, 0)
            || destination == get_value(&cups, &first, 1)
            || destination == get_value(&cups, &first, 2)
        {
            destination = if destination <= 1 { 1_000_000 } else { destination - 1 };
        }

        // Move the first item
        cups.insert(last.unwrap(), first.clone());
        last = first;
        first = cups.get(&first.unwrap()).unwrap().clone();

        // Move first 3 items
        let after = cups.get(&destination).unwrap().clone();
        cups.insert(destination, first.clone());
        let last_of_three = get_value(&cups, &first, 1);
        first = cups.get(&last_of_three).unwrap().clone();
        cups.insert(last_of_three, after);
    }

    let x = cups.get(&1).unwrap().unwrap();
    return x * cups.get(&x).unwrap().unwrap();
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from("389125467");
        assert_eq!(run(input), 149245887792);
    }
}
