use std::collections::HashMap;

pub fn run(input: String) -> u32 {
    let limit = 30_000_000;
    let mut indices: HashMap<u32, (Option<u32>, u32)> = HashMap::new();
    let mut last_num = 0;
    let mut turn = 0;

    input.split_terminator(',').enumerate().for_each(|(i, n)| {
        last_num = n.trim().parse::<u32>().expect("Spoken number should be a valid integer");
        turn = i as u32;
        indices.insert(last_num, (None, turn));
    });

    while turn < limit - 1 {
        turn += 1;
        let (prev_index, last_index) = indices.get(&last_num).unwrap_or(&(None, 0));
        last_num = match prev_index {
            Some(i) => last_index - i,
            None => 0,
        };
        indices.insert(
            last_num,
            (
                if indices.contains_key(&last_num) {
                    Some(indices.get(&last_num).unwrap().1)
                } else {
                    None
                },
                turn,
            ),
        );
    }

    last_num
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly1() {
        assert_eq!(run(String::from("0,3,6")), 175594);
    }

    #[test]
    fn should_run_correctly2() {
        assert_eq!(run(String::from("1,3,2")), 2578);
    }
    #[test]
    fn should_run_correctly3() {
        assert_eq!(run(String::from("2,1,3")), 3544142);
    }
    #[test]
    fn should_run_correctly4() {
        assert_eq!(run(String::from("1,2,3")), 261214);
    }
    #[test]
    fn should_run_correctly5() {
        assert_eq!(run(String::from("2,3,1")), 6895259);
    }
    #[test]
    fn should_run_correctly6() {
        assert_eq!(run(String::from("3,2,1")), 18);
    }
    #[test]
    fn should_run_correctly7() {
        assert_eq!(run(String::from("3,1,2")), 362);
    }
}
