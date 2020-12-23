use std::collections::VecDeque;

pub fn run(input: String) -> u32 {
    let mut cups: VecDeque<u32> = input.trim().chars().map(|cup| cup.to_digit(10).unwrap()).collect();

    for _ in 0..100 {
        let mut destination = *cups.get(0).unwrap() - 1;
        while destination == 0 || destination == *cups.get(1).unwrap() || destination == *cups.get(2).unwrap() || destination == *cups.get(3).unwrap()
        {
            destination = if destination <= 1 { 9 } else { destination - 1 };
        }

        cups.rotate_left(1);

        let insert_pos = cups.iter().position(|cup| *cup == destination).unwrap();
        for _ in 0..3 {
            let cup_to_move = cups.pop_front().unwrap();
            cups.insert(insert_pos, cup_to_move);
        }
    }

    let one_pos = cups.iter().position(|cup| *cup == 1).unwrap();
    cups.rotate_left(one_pos);
    cups.pop_front();

    cups.iter().rev().enumerate().fold(0, |acc, (i, cup)| acc + 10u32.pow(i as u32) * *cup)
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn should_run_correctly() {
        let input = String::from("389125467");
        assert_eq!(run(input), 67384529);
    }
}
